use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

/// A very thin wrapper around its inner string, to prevent accidental logging of the access token.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccessToken(pub String);

impl Debug for AccessToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AccessToken").field(&"redacted").finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The ID is unique across all users, and identifies this specific user.
    pub id: Uuid,

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

    /// The access token for the user.
    pub access_token: AccessToken,
}

impl Into<proto::auth::User> for User {
    fn into(self) -> proto::auth::User {
        proto::auth::User {
            id: self.id.to_string(),
            username: self.username,
            email: self.email,
            display_name: self.display_name,
        }
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.0.as_bytes()
    }
}
