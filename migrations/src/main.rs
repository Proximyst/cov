use clap::Parser;
use eyre::{Context, Result, eyre};
use sqlx::{
    Connection, PgConnection,
    postgres::{PgConnectOptions, PgSslMode},
    query,
};
use std::{path::PathBuf, str::FromStr};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Cov is a code coverage tool. This is the server running the entire application.
#[derive(Debug, Parser)]
#[command(name = "cov-migrations", about = None, long_about)]
struct Args {
    /// Sets the logger filter.
    #[arg(long, default_value = "warn,cov_migrations=info", env = "RUST_LOG")]
    logger: String,

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv().ok();

    color_eyre::install()?;
    let args: Args = Args::parse();
    setup_logging(&args.logger).wrap_err("failed to init logging")?;

    let mut opts = PgConnectOptions::from_str(&args.postgres)
        .wrap_err("failed to parse connection options")?;

    opts = apply_ca_cert(args.postgres_ca_certificate, opts)
        .await
        .wrap_err("failed to apply the CA certificate")?;

    let mut conn = PgConnection::connect_with(&opts)
        .await
        .wrap_err("failed to connect to database")?;

    let _ = query("SELECT 1")
        .execute(&mut conn)
        .await
        .wrap_err("failed to execute aliveness query")?;

    cov_migrations::MIGRATOR
        .run(&mut conn)
        .await
        .wrap_err("failed to run migrations")?;
    info!("migrations ran successfully");

    Ok(())
}

async fn apply_ca_cert(
    path: Option<PathBuf>,
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

fn setup_logging(filter: &str) -> Result<()> {
    let env_filter =
        EnvFilter::from_str(filter).wrap_err("could not parse RUST_LOG environment variable")?;
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| eyre!(e))
        .wrap_err("failed to initialise logger")?;

    Ok(())
}
