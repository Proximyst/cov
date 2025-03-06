use deadpool_postgres::{Client, Pool, PoolError};
use eyre::{Context, Result};
use futures::{FutureExt, future::OptionFuture};
use metrics::counter;
use native_tls::{Certificate, TlsConnector};
use proto::health;
use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{Interval, interval},
};
use tokio_postgres::NoTls;
use tracing::{debug, info, warn};

use crate::health::Component;

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

    let manager_config = deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
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
            deadpool_postgres::Manager::from_config(cfg, tls, manager_config)
        }
        None => deadpool_postgres::Manager::from_config(cfg, NoTls, manager_config),
    };
    let pool = deadpool_postgres::Pool::builder(manager)
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
pub enum Command {}

async fn command_actor(
    pool: Pool,
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
        let conn = loop {
            match pool.get().await {
                Ok(conn) => break conn,
                Err(PoolError::Closed) => {
                    info!("database pool closed; shutting down database actor");
                    return;
                }
                Err(err) => {
                    counter!("database_connection_failures").increment(1);
                    tracing::warn!(error = ?err, "failed to get database connection. trying again");
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
        };
        match process(conn, command).await {
            Ok(()) => {
                counter!("database_commands_processed", "outcome" => "success").increment(1);
            }
            Err(err) => {
                counter!("database_commands_processed", "outcome" => "failure").increment(1);
            }
        }
    }

    info!("database channels closed; shutting down database actor");
}

async fn process(conn: Client, command: Command) -> Result<()> {
    match command {}

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
                counter!("database_health_check_successes", "database" => db).increment(1);
                true
            }
            Err(err) => {
                counter!("database_health_check_failures", "database" => db).increment(1);
                warn!(db, ?err, "failed to check database health");
                false
            }
        },
        Err(err) => {
            counter!("database_health_check_failures", "database" => db).increment(1);
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
            counter!("database_health_check", "state" => "healthy").increment(1);
            health::State::Healthy
        } else {
            counter!("database_health_check", "state" => "unhealthy").increment(1);
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

pub async fn spawn_database_actor(
    set: &mut JoinSet<()>,
    args: PostgresArgs,
    health_tx: mpsc::Sender<(Component, health::State)>,
) -> Result<()> {
    let pools = create_pools(&args).await?;
    set.spawn(health_check_actor(pools.clone(), health_tx));

    Ok(())
}
