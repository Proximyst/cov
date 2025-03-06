use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The server is alive. It contains when the request was processed, such that latency can be measured.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Pong {
    /// The time at which the request was processed by our handlers.
    /// This allows the client to measure latency between the client and server after all middlewares and similar are complete.
    pub processed_timestamp: DateTime<Utc>,
}
