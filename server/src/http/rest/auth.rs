use crate::database::Database;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

/// A logged in user.
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    /// The ID is unique across all users, and identifies this specific user.
    pub id: Uuid,

    /// The username of the user. This is a human-readable name.
    /// It is not unique across all users, and may change.
    ///
    /// This is usually mapped to the username of the service the user logs in with, e.g. their GitHub or GitLab username.
    pub username: String,

    /// The hashed password of this user.
    password: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    db: Database,
}

impl Backend {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] crate::database::users::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let Some(user) = crate::database::users::get_user_by_username(&self.db, &creds.username)
            .await
            .map_err(Error::Database)?
        else {
            return Ok(None);
        };

        let valid = crate::database::users::verify_user_access(
            &self.db,
            user.id,
            creds.password.as_bytes(),
        )
        .await?;

        Ok(Some(user).filter(|_| valid).map(|u| User {
            id: u.id,
            username: u.username,
            password: u.password,
        }))
    }

    async fn get_user(&self, id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(
            crate::database::users::get_user_by_id(&self.db, id.to_owned())
                .await
                .map_err(Error::Database)?
                .map(|u| User {
                    id: u.id,
                    username: u.username,
                    password: u.password,
                }),
        )
    }
}

pub type Session = axum_login::AuthSession<Backend>;
