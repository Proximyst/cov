use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An authentication token to gain access to the API.
/// It is intended to be passed in a self-signed JWT.
/// Due to this being an implementation detail, it does not hold itself to the standard versioning guarantees: the format may change at any time in any way. The server will deal with these changes sensibly.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Token {
    /// The ID is unique across all users and tokens.
    /// It is used to identify the token.
    ///
    /// The ID format is unspecified. It may change from time to time, so treat it as an arbitrary string.
    #[schemars(length(min = 1, max = 64))]
    pub id: String,
    /// The user with which this token is associated.
    /// This is optional, as the token may not be associated with a user, rather a repository or similar.
    #[schemars(length(min = 1, max = 64))]
    pub user_id: Option<String>,
    /// This token is invalid until this time.
    /// If used before this time, it will be revoked.
    pub not_before: DateTime<Utc>,
    /// This token is invalid after this time.
    /// If used after this time, it will be revoked.
    pub not_after: DateTime<Utc>,
}

/// A specific user on the instance. Usually the logged in user.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct User {
    /// The ID is unique across all users, and identifies this specific user.
    ///
    /// The ID format is unspecified. It may change from time to time, so treat it as an arbitrary string.
    #[schemars(length(min = 1, max = 64))]
    pub id: String,

    /// The username of the user. This is a human-readable name.
    /// It is not unique across all users, and may change.
    ///
    /// This is usually mapped to the username of the service the user logs in with, e.g. their GitHub or GitLab username.
    pub username: String,

    /// The email of the user.
    /// It is not unique across all users, and may change.
    ///
    /// Not all users have access to others' emails. The logged in user always has access to their own email.
    pub email: Option<String>,

    /// The display name of the user. This is a human-readable name.
    /// It is not unique across all users, and may change.
    ///
    /// This is usually the user's preferred name on their service, e.g. their GitHub or GitLab display name.
    /// If none is known on the service, this is equal to the username.
    pub display_name: String,
}
