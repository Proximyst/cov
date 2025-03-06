use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The health of the entire system.
/// Along with the body of this endpoint, it may return HTTP 200 for a healthy system and HTTP 500 for an unhealthy one.
///
/// This is not necessarily served on the public API, as it is somewhat sensitive information.
/// If this is the case, the health API will simply not exist (and instead return 404, or another status code as defined by a potential reverse proxy blocking the path).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    /// The last time a field might have changed.
    pub last_update: DateTime<Utc>,

    /// The components in the system along with their health state.
    /// The key is a component name. This is not stable across versions.
    pub components: HashMap<String, State>,
}

/// The state of a singular component in the system.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub enum State {
    /// The component is healthy and ready to serve its part.
    Healthy,

    /// The component is unhealthy for a specified reason.
    /// It may become healthy given some time.
    Unhealthy(String),

    /// The component's current health state is unknown.
    /// This usually means it has not yet started up.
    #[default]
    Unknown,
}
