use super::pools::Pools;
use crate::health::Component;
use futures::{FutureExt, future::OptionFuture};
use metrics::counter;
use proto::health;
use sqlx::{PgPool, query};
use std::time::Duration;
use tokio::{sync::mpsc, time::interval};
use tracing::{info, trace, warn};

pub async fn health_check_actor(pools: Pools, health_tx: mpsc::Sender<(Component, health::State)>) {
    let mut interval = interval(Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        let _ = interval.tick().await;

        let rw_healthy = health_check(pools.read_write(), "read-write");
        let ro_healthy =
            OptionFuture::from(pools.read_only_opt().map(|r| health_check(r, "read-only")))
                .map(|f| f.unwrap_or(Ok(())));
        // we use try_join to fail fast if one is unhealthy
        let healthy = tokio::try_join!(rw_healthy, ro_healthy).is_ok();

        let health = if healthy {
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
    }
}

async fn health_check(client: PgPool, db: &'static str) -> Result<(), ()> {
    match query("SELECT 1").execute(&client).await {
        Ok(_) => {
            counter!("cov.database.health_check",
                    "outcome" => "success",
                    "database" => db)
            .increment(1);
            trace!(db, "database health check succeeded");
            Ok(())
        }
        Err(err) => {
            counter!("cov.database.health_check",
                    "outcome" => "failure",
                    "database" => db,
                    "reason" => "select-error")
            .increment(1);
            warn!(db, ?err, "failed to check database health");
            Err(())
        }
    }
}
