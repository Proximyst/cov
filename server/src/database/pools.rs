use eyre::{Context, Result};
use futures::{FutureExt, future::OptionFuture};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
};
use std::{path::Path, str::FromStr};

/// Pools for the database connections.
///
/// This can be cloned, just like individual [`PgPool`]s can.
#[derive(Debug, Clone)]
pub struct Pools {
    read_write: PgPool,
    read_only: Option<PgPool>,
}

impl Pools {
    pub fn new(read_write: PgPool, read_only: Option<PgPool>) -> Self {
        Self {
            read_write,
            read_only,
        }
    }

    /// Get the read/write connection pool.
    pub fn read_write(&self) -> PgPool {
        self.read_write.clone()
    }

    /// Get the read-only connection pool, if configured.
    /// If a read replica is configured, this will return the pool to the read replica.
    /// Otherwise, it will return the read/write pool, just like [`Self::read_write`].
    pub fn read_only(&self) -> PgPool {
        if let Some(pool) = &self.read_only {
            pool.clone()
        } else {
            self.read_write()
        }
    }

    /// Get the read-only connection pool, if configured.
    pub fn read_only_opt(&self) -> Option<PgPool> {
        self.read_only.clone()
    }
}

async fn create_pool(
    conn_string: &str,
    pool_sz: u32,
    ca_cert: Option<&Path>,
    run_migrations: bool,
) -> Result<PgPool> {
    let mut opts =
        PgConnectOptions::from_str(conn_string).wrap_err("failed to parse connection options")?;

    opts = apply_ca_cert(ca_cert, opts)
        .await
        .wrap_err("failed to apply the CA certificate")?;

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(pool_sz)
        .connect_with(opts)
        .await
        .wrap_err("failed to connect to database")?;

    if run_migrations {
        cov_migrations::MIGRATOR
            .run(&pool)
            .await
            .wrap_err("failed to run migrations")?;
    }

    Ok(pool)
}

async fn apply_ca_cert(
    path: Option<&Path>,
    mut opts: PgConnectOptions,
) -> Result<PgConnectOptions> {
    let Some(path) = path else {
        return Ok(opts);
    };

    if !matches!(
        opts.get_ssl_mode(),
        PgSslMode::VerifyCa | PgSslMode::VerifyFull
    ) {
        opts = opts.ssl_mode(PgSslMode::VerifyCa);
    }

    if path.ends_with(".pem") {
        let pem = tokio::fs::read(path)
            .await
            .wrap_err("failed to read PEM file")?;

        opts = opts.ssl_root_cert_from_pem(pem);
    } else {
        opts = opts.ssl_root_cert(&path);
    }

    Ok(opts)
}

pub(super) async fn create_pools(args: &super::PostgresArgs) -> Result<Pools> {
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

    Ok(Pools::new(rw, ro))
}
