use super::{Database, Db};
use chrono::Utc;
use eyre::{Context, Result};
use futures::FutureExt;
use metrics::counter;
use sqlx::query;
use std::time::Duration;
use tokio::time::{MissedTickBehavior, interval};
use tracing::{debug, warn};

pub async fn periodic_cleaner(db: Database) {
    let mut interval = interval(Duration::from_secs(60));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        let _ = interval.tick().await;

        counter!("cov.database.cleaner.ticks").increment(1);
        if let Err(err) = clean(&db).await {
            counter!("cov.database.cleaner.outcome", "outcome" => "failure").increment(1);
            warn!(?err, "failed to clean database");
        } else {
            counter!("cov.database.cleaner.outcome", "outcome" => "success").increment(1);
            debug!("cleaned database");
        }
    }
}

async fn clean(db: &Database) -> Result<()> {
    let now = Utc::now();
    let rw = db.read_write();

    let clean_tokens = query!("DELETE FROM user_tokens WHERE expiry <= $1", now)
        .execute(&rw)
        .map(|r| r.wrap_err("failed to clean user tokens"));

    let clean_oauth2 = query!("DELETE FROM user_oauth2 WHERE expiry <= $1", now)
        .execute(&rw)
        .map(|r| r.wrap_err("failed to clean user oauth2 tokens"));

    tokio::try_join!(clean_tokens, clean_oauth2)?;

    Ok(())
}
