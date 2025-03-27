mod database;
mod health;
mod http;
mod report;

use clap::Parser;
use eyre::{Context, Result, eyre};
use metrics::{counter, gauge};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use proto::health::Health;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};
use tokio::{sync::watch, task::JoinSet};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Cov is a code coverage tool. This is the server running the entire application.
#[derive(Debug, Parser)]
#[command(name = "cov", about = None, long_about)]
struct Args {
    /// Sets the logger filter.
    #[arg(long, default_value = "warn,cov_server=info", env = "RUST_LOG")]
    logger: String,

    #[command(flatten, next_help_heading = "HTTP servers")]
    http: http::HttpArgs,

    #[command(flatten, next_help_heading = "Postgres")]
    postgres: database::PostgresArgs,
}

#[tokio::main]
async fn main() -> Result<()> {
    let startup = Instant::now();
    let _ = dotenvy::dotenv().ok();

    color_eyre::install()?;
    let args: Args = Args::parse();
    setup_logging(&args.logger).wrap_err("failed to init logging")?;

    let mut join_set = JoinSet::new();

    let recorder = PrometheusBuilder::new()
        .upkeep_timeout(Duration::from_secs(120))
        .install_recorder()
        .wrap_err("could not prepare Prometheus exporter")?;
    gauge!("cov.up").set(1);
    join_set.spawn(prometheus_upkeep_actor(recorder.clone()));
    info!("metrics backend initialised");

    let (component_health_tx, health_rx) = health::spawn_tracking_actor(&mut join_set);
    tokio::spawn(till_ready(startup, health_rx.clone()));
    info!("health actor initialised");

    http::spawn_health_actor(
        &mut join_set,
        &args.http,
        component_health_tx.clone(),
        health_rx,
        recorder.clone(),
    );
    info!("health http actor initialised");

    let db = database::start_database(&mut join_set, args.postgres, component_health_tx.clone())
        .await
        .wrap_err("failed to start database actor")?;
    info!("database actor initialised");
    create_admin_user(&db)
        .await
        .wrap_err("failed to create default admin user if necessary")?;

    http::spawn_rest_actor(&mut join_set, &args.http, component_health_tx.clone())
        .await
        .wrap_err("failed to spawn rest actor")?;
    info!("rest http actor initialised");

    let _ = join_set.join_next().await;
    info!("a task in the join set completed; shutting down");
    Ok(())
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

async fn prometheus_upkeep_actor(recorder: PrometheusHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(120));
    loop {
        interval.tick().await;
        counter!("cov.prometheus.upkeep_ticks").increment(1);
        recorder.run_upkeep();
    }
}

async fn till_ready(startup: Instant, mut health: watch::Receiver<Health>) {
    while let Ok(()) = health.changed().await {
        if health
            .borrow()
            .components
            .values()
            .any(|s| !matches!(s, proto::health::State::Healthy))
        {
            continue;
        }

        let now = Instant::now();
        let elapsed = now - startup;
        gauge!("cov.startup.duration", "unit" => "nanos").set(elapsed.as_nanos() as f64);
        gauge!("cov.startup.duration", "unit" => "millis").set(elapsed.as_millis() as f64);
        return;
    }
}

async fn create_admin_user(db: &impl database::Db) -> Result<()> {
    if database::users::has_users(db)
        .await
        .wrap_err("failed to check if any users exist")?
    {
        // We already have users, so we don't need to create an admin user.
        return Ok(());
    }

    let _ = database::users::create_user(db, "admin@localhost", "admin", "admin", b"admin")
        .await
        .wrap_err("failed to create 'admin' user")?;
    info!("created a default admin user with 'admin' password");

    Ok(())
}
