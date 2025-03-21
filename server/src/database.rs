pub mod create_org;
pub mod create_repo;
pub mod model;

use crate::health::Component;
use deadpool_postgres::{Client, Pool, PoolError};
use eyre::{Context, Result};
use futures::{FutureExt, future::OptionFuture};
use metrics::counter;
use model::Service;
use native_tls::{Certificate, TlsConnector};
use proto::health;
use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
    time::interval,
};
use tokio_postgres::NoTls;
use tracing::{debug, error, info, warn};

refinery::embed_migrations!("./src/migrations");

#[derive(Debug, clap::Args)]
pub struct PostgresArgs {
    /// The connection string for the Postgres database.
    ///
    /// The format is documented in good detail here: https://docs.rs/tokio-postgres/0.7.13/tokio_postgres/config/struct.Config.html
    #[arg(
        long,
        default_value = "user=cov password=cov dbname=cov host=localhost port=5432",
        env
    )]
    postgres: String,

    /// The same as --postgres, except for a read-replica.
    ///
    /// If given, the application will use this connection string for read-only queries and the other for mostly write queries.
    #[arg(long, env)]
    postgres_read_replica: Option<String>,

    /// The size of the Postgres connection pool.
    ///
    /// This is the maximum number of connections that can be open at the same time.
    /// Making it larger does not necessarily mean more performance, as it can lead to more contention and be bottlenecked elsewhere.
    #[arg(long, default_value_t = 30, env)]
    postgres_pool_size: usize,

    /// The CA certificate for the Postgres connection. If not provided, the connection will not use SSL.
    ///
    /// If you do not specify this while also requiring SSL, the application will fail to start.
    /// If specified, the connection will require SSL by overriding the `sslmode` option.
    /// The same certificate will be used for all connections to all replicas.
    ///
    /// The file must be in PEM format. DER is not supported.
    #[arg(long, env)]
    postgres_ca_certificate: Option<PathBuf>,
}

/// Pools for the database connections.
///
/// This can be cloned, just like individual [Pool]s can.
#[derive(Debug, Clone)]
pub struct Pools {
    read_write: Pool,
    read_only: Option<Pool>,
}

impl Pools {
    /// Acquire a read/write connection to the database.
    pub async fn read_write(&self) -> Result<Client, PoolError> {
        self.read_write.get().await
    }

    /// Acquire a read-only connection to the database.
    /// If a read replica is configured, this will return a connection to the read replica.
    /// Otherwise, it will return a connection to the read/write pool, just like [Self::read_write].
    pub async fn read_only(&self) -> Result<Client, PoolError> {
        if let Some(pool) = &self.read_only {
            pool.get().await
        } else {
            self.read_write.get().await
        }
    }
}

async fn create_pool(
    conn_string: &str,
    pool_sz: usize,
    ca_cert: Option<&Path>,
    run_migrations: bool,
) -> Result<Pool> {
    let mut cfg = tokio_postgres::Config::from_str(conn_string)
        .wrap_err("failed to parse connection string")?;

    use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod::Fast};
    let manager_config = ManagerConfig {
        recycling_method: Fast,
    };
    let manager = match ca_cert {
        Some(path) => {
            debug!(existing = ?cfg.get_ssl_mode(), "CA certificate was provided; requiring SSL");
            cfg.ssl_mode(tokio_postgres::config::SslMode::Require);

            let cert = tokio::fs::read(&path)
                .await
                .wrap_err("failed to read CA certificate")?;
            let cert =
                Certificate::from_pem(&cert).wrap_err("failed to parse CA certificate as PEM")?;
            let connector = TlsConnector::builder()
                .add_root_certificate(cert)
                .build()
                .wrap_err("failed to build TLS connector")?;
            let tls = postgres_native_tls::MakeTlsConnector::new(connector);
            Manager::from_config(cfg, tls, manager_config)
        }
        None => Manager::from_config(cfg, NoTls, manager_config),
    };
    let pool = Pool::builder(manager)
        .max_size(pool_sz)
        .build()
        .wrap_err("failed to build deadpool")?;

    let mut conn = pool
        .get()
        .await
        .wrap_err("could not establish connection to database")?;
    if run_migrations {
        // We always want to establish a connection to make sure the connection string is valid.
        // However, we don't want to run migrations on read-only replicas.
        migrations::runner()
            .run_async(conn.deref_mut().deref_mut())
            .await
            .wrap_err("failed to run migrations")?;
    }
    drop(conn);

    Ok(pool)
}

#[derive(Debug)]
pub enum Command {
    CreateOrganisation(
        create_org::CreateOrganisation,
        oneshot::Sender<Result<(), create_org::Error>>,
    ),
    CreateRepository(
        create_repo::CreateRepository,
        oneshot::Sender<Result<(), create_repo::Error>>,
    ),
}

async fn command_actor(
    pools: Pools,
    mut high_priority_commands: mpsc::Receiver<Command>,
    mut low_priority_commands: mpsc::Receiver<Command>,
) {
    while let Some(command) = tokio::select! {
        // We want the user queue to be read first, as it's more important.
        // The user latency is always more impactful than waiting a couple extra milliseconds on the background queue.
        biased;

        cmd = high_priority_commands.recv() => cmd,
        cmd = low_priority_commands.recv() => cmd,
    } {
        let pools = pools.clone();
        tokio::spawn(async move {
            match process(pools, command).await {
                Ok(()) => {
                    counter!("cov.database.commands_processed", "outcome" => "success")
                        .increment(1);
                }
                Err(err) => {
                    counter!("cov.database.commands_processed", "outcome" => "failure")
                        .increment(1);
                    error!(?err, "failed to process command");
                }
            }
        });
    }

    info!("database channels closed; shutting down database actor");
}

async fn process(pools: Pools, command: Command) -> Result<()> {
    match command {
        Command::CreateOrganisation(cmd, reply) => {
            create_org::create_organisation(pools, cmd, reply).await;
        }
        Command::CreateRepository(cmd, reply) => {
            create_repo::create_repository(pools, cmd, reply).await;
        }
    }

    Ok(())
}

async fn create_pools(args: &PostgresArgs) -> Result<Pools> {
    let rw = create_pool(
        &args.postgres,
        args.postgres_pool_size,
        args.postgres_ca_certificate.as_deref(),
        true,
    )
    .map(|f| f.wrap_err("failed to create read-write database connection pool"));
    let ro: OptionFuture<_> = args
        .postgres_read_replica
        .as_deref()
        .map(|conn| {
            create_pool(
                conn,
                args.postgres_pool_size,
                args.postgres_ca_certificate.as_deref(),
                false,
            )
        })
        .into();
    let ro = ro.map(|o| {
        o.transpose()
            .wrap_err("failed to create read-only database connection pool")
    });
    let (rw, ro) = tokio::try_join!(rw, ro)?;

    Ok(Pools {
        read_write: rw,
        read_only: ro,
    })
}

async fn health_check(pool: &Pool, db: &'static str) -> bool {
    match pool.get().await {
        Ok(conn) => match conn.query("SELECT 1", &[]).await {
            Ok(_) => {
                counter!("cov.database.health_check",
                    "outcome" => "success",
                    "database" => db)
                .increment(1);
                true
            }
            Err(err) => {
                counter!("cov.database.health_check",
                    "outcome" => "failure",
                    "database" => db,
                    "reason" => "select-error")
                .increment(1);
                warn!(db, ?err, "failed to check database health");
                false
            }
        },
        Err(err) => {
            counter!("cov.database.health_check",
                "outcome" => "failure",
                "database" => db,
                "reason" => "acquire-connection")
            .increment(1);
            warn!(
                db,
                ?err,
                "failed to acquire connection to check database health",
            );
            false
        }
    }
}

async fn health_check_actor(pools: Pools, health_tx: mpsc::Sender<(Component, health::State)>) {
    let mut interval = interval(Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        let rw_healthy = health_check(&pools.read_write, "read-write").await;

        let ro_healthy: OptionFuture<_> = pools
            .read_only
            .as_ref()
            .map(|p| health_check(p, "read-only"))
            .into();
        let ro_healthy = ro_healthy.await.unwrap_or(true);

        let health = if rw_healthy && ro_healthy {
            counter!("cov.database.health_check", "state" => "healthy").increment(1);
            health::State::Healthy
        } else {
            counter!("cov.database.health_check", "state" => "unhealthy").increment(1);
            health::State::Unhealthy(String::from("database health check failed"))
        };
        let Ok(()) = health_tx.send((Component::Database, health)).await else {
            info!(
                "database health check succeeded, but failed to send health state. shutting down health check actor",
            );
            return;
        };

        let _ = interval.tick().await;
    }
}

/// The priority of a command determines how urgently it should be processed.
/// If a high-priority command exists in the queue, it is always processed before a low-priority one.
/// That means that a low-priority command can be delayed indefinitely if there are always high-priority commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// A high-priority command, usually from a direct user action.
    High,
    /// A low-priority command, usually from a background task or worker thread.
    Low,
}

#[derive(Debug, Clone)]
pub struct Database {
    high_prio: mpsc::Sender<Command>,
    low_prio: mpsc::Sender<Command>,
}

impl Database {
    pub async fn create_organisation(
        &self,
        priority: Priority,
        svc: Service,
        name: &str,
    ) -> Result<(), create_org::Error> {
        let (tx, rx) = oneshot::channel();
        // The receiver cannot be closed, as we still own it.
        let _ = self
            .queue(priority)
            .send(Command::CreateOrganisation(
                create_org::CreateOrganisation::new(name, svc),
                tx,
            ))
            .await;
        rx.await.map_err(|_| create_org::Error::ReplyClosed)?
    }

    pub async fn create_repository(
        &self,
        priority: Priority,
        svc: Service,
        org_name: &str,
        name: &str,
    ) -> Result<(), create_repo::Error> {
        let (tx, rx) = oneshot::channel();
        // The receiver cannot be closed, as we still own it.
        let _ = self
            .queue(priority)
            .send(Command::CreateRepository(
                create_repo::CreateRepository::new(svc, org_name, name),
                tx,
            ))
            .await;
        rx.await.map_err(|_| create_repo::Error::ReplyClosed)?
    }

    fn queue(&self, prio: Priority) -> &mpsc::Sender<Command> {
        match prio {
            Priority::High => &self.high_prio,
            Priority::Low => &self.low_prio,
        }
    }
}

pub async fn spawn_database_actor(
    set: &mut JoinSet<()>,
    args: PostgresArgs,
    health_tx: mpsc::Sender<(Component, health::State)>,
) -> Result<Database> {
    let pools = create_pools(&args).await?;
    set.spawn(health_check_actor(pools.clone(), health_tx));

    let (htx, hrx) = mpsc::channel(4);
    let (ltx, lrx) = mpsc::channel(1);
    let db = Database {
        high_prio: htx,
        low_prio: ltx,
    };
    set.spawn(command_actor(pools, hrx, lrx));

    Ok(db)
}
