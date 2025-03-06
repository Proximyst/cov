use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An authentication token to gain access to the API.
/// It is intended to be passed in a self-signed JWT.
/// Due to this being an implementation detail, it does not hold itself to the standard versioning guarantees: the format may change at any time in any way. The server will deal with these changes sensibly.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Token {
    /// The ID is unique across all users and tokens.
    /// It is used to identify the token.
    /// It is always <= 64 ASCII characters long.
    ///
    /// The ID format is unspecified. It may change from time to time, so treat it as an arbitrary string.
    #[schemars(length(min = 1, max = 64))]
    pub id: String,
    /// The user with which this token is associated.
    /// This is optional, as the token may not be associated with a user, rather a repository or similar.
    pub user_id: Option<Uuid>,
    /// This token is invalid until this time.
    /// If used before this time, it will be revoked.
    pub not_before: DateTime<Utc>,
    /// This token is invalid after this time.
    /// If used after this time, it will be revoked.
    pub not_after: DateTime<Utc>,
}
