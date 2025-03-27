mod cleaner;
mod encryption;
mod health;
mod pools;
pub mod users;

use crate::health::Component;
use eyre::Result;
use pools::Pools;
use sqlx::{PgPool, postgres::PgDatabaseError};
use std::path::PathBuf;
use tokio::{sync::mpsc, task::JoinSet};

#[derive(Debug, clap::Args)]
pub struct PostgresArgs {
    /// The connection string for the Postgres database.
    ///
    /// The format is documented in good detail here: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html
    #[arg(long, default_value = "postgres://cov:cov@localhost:5432/cov", env)]
    postgres: String,

    /// The CA certificate for the Postgres connection. If not provided, the connection will not use SSL.
    ///
    /// If you do not specify this while also requiring SSL, the application will fail to start.
    /// If specified, the connection will require SSL by overriding the `sslmode` option.
    /// The same certificate will be used for all connections to all replicas.
    ///
    /// This should be a path to a `.crt` or `.pem` file. If it ends with `.pem` (it MUST end with `.pem`; case-sensitive), it will be treated as a PEM file.
    /// Anything else is assumed to be a `.crt` file.
    #[arg(long, env)]
    postgres_ca_certificate: Option<PathBuf>,

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
    postgres_pool_size: u32,

    /// The KEK for the encrypted data.
    ///
    /// This is a hex-encoded 256-bit key used to encrypt and decrypt data.
    /// To generate a new KEK, do: `openssl rand -hex 32`
    ///
    /// If multiple replicas are run with the same database, this must be equal amongst them all.
    // TODO: We need a way to regenerate this key without losing all data.
    #[arg(long, env)]
    database_kek: encryption::Key,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failure in database")]
    Postgres(#[from] PgDatabaseError),

    #[error("encryption failure")]
    Encryption(#[from] encryption::Error),
}

#[derive(Debug, Clone)]
pub struct Database {
    kek: encryption::Key,
    pools: Pools,
}

/// A trait defining the database operations.
///
/// This exists to be able to mock the database in tests.
pub trait Db {
    /// Get the read/write connection pool.
    fn read_write(&self) -> PgPool;

    /// Get the read-only connection pool, if configured.
    /// If a read replica is configured, this will return the pool to the read replica.
    /// Otherwise, it will return the read/write pool, just like [`Self::read_write`].
    fn read_only(&self) -> PgPool;

    /// Get the encryption key for the database.
    fn kek(&self) -> encryption::Key;
}

impl Db for Database {
    fn read_write(&self) -> PgPool {
        self.pools.read_write()
    }

    fn read_only(&self) -> PgPool {
        self.pools.read_only()
    }

    fn kek(&self) -> encryption::Key {
        self.kek.clone()
    }
}

pub async fn start_database(
    set: &mut JoinSet<()>,
    args: PostgresArgs,
    health_tx: mpsc::Sender<(Component, proto::health::State)>,
) -> Result<Database> {
    let pools = pools::create_pools(&args).await?;
    set.spawn(health::health_check_actor(pools.clone(), health_tx));

    let db = Database {
        kek: args.database_kek,
        pools,
    };
    set.spawn(cleaner::periodic_cleaner(db.clone()));

    Ok(db)
}
