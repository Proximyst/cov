use super::Db;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use sqlx::{query, query_as};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("password failure: {0}")]
    Hash(#[from] argon2::password_hash::Error),

    #[error("failed to join with forked work: {0}")]
    Join(#[from] tokio::task::JoinError),
}

type Result<T, E = Error> = std::result::Result<T, E>;

/// A user.
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    /// The user's hashed password.
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Check if any users exist at all.
pub async fn has_users(db: &impl Db) -> Result<bool> {
    Ok(
        query!(r#"SELECT EXISTS (SELECT 1 FROM users LIMIT 1) AS "exists!""#)
            .fetch_one(&db.read_only())
            .await
            .map(|row| row.exists)?,
    )
}

/// Create a new user.
pub async fn create_user(
    db: &impl Db,
    email: &str,
    username: &str,
    display_name: &str,
    password: &[u8],
) -> Result<User> {
    let id = Uuid::now_v7();
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed = argon2.hash_password(password, &salt)?.to_string();

    Ok(query_as!(
        User,
        "INSERT INTO users (id, email, username, display_name, password)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *",
        id,
        email,
        username,
        display_name,
        hashed,
    )
    .fetch_one(&db.read_write())
    .await?)
}

/// Find an existing user by their username.
pub async fn get_user_by_username(db: &impl Db, username: &str) -> Result<Option<User>> {
    Ok(
        query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&db.read_only())
            .await?,
    )
}

/// Find an existing user by their ID.
pub async fn get_user_by_id(db: &impl Db, id: Uuid) -> Result<Option<User>> {
    Ok(query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&db.read_only())
        .await?)
}

/// Update the user's password.
pub async fn update_user_password(db: &impl Db, user_id: Uuid, password: &[u8]) -> Result<()> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed = argon2.hash_password(password, &salt)?.to_string();

    query!(
        "UPDATE users
        SET password = $2, updated_at = NOW()
        WHERE id = $1",
        user_id,
        hashed,
    )
    .execute(&db.read_write())
    .await?;

    Ok(())
}

/// Verify whether the input password matches the stored password for a user.
pub async fn verify_user_password(
    db: &impl Db,
    user_id: Uuid,
    password: impl Into<Box<[u8]>>,
) -> Result<bool> {
    let row = query!("SELECT password FROM users WHERE id = $1", user_id)
        .fetch_one(&db.read_only())
        .await?;

    let password = password.into();
    spawn_blocking(move || {
        PasswordHash::new(&row.password)
            .map_err(Error::from)
            .and_then(|hash| {
                Argon2::default()
                    .verify_password(&password, &hash)
                    .map(|_| true)
                    .or(Ok(false))
            })
    })
    .await?
}

/// Checks if the password is valid for the user.
///
/// This checks both the user's password and any active tokens.
pub async fn verify_user_access(
    db: &impl Db,
    user_id: Uuid,
    password: impl Into<Box<[u8]>>,
) -> Result<bool> {
    let ro = db.read_only();
    let mut tokens = query!(
        r#"SELECT password AS "access_token!" FROM users WHERE id = $1
        UNION
        SELECT access_token FROM user_tokens WHERE user_id = $1 AND expiry > NOW()"#,
        user_id
    )
    .fetch(&ro);

    let password = Arc::new(password.into());
    while let Some(token) = tokens.next().await {
        let token = token?;

        let password = password.clone();
        let matching = spawn_blocking(move || {
            PasswordHash::new(&token.access_token)
                .map_err(Error::from)
                .and_then(|hash| {
                    Argon2::default()
                        .verify_password(&password, &hash)
                        .map(|_| true)
                        .or(Ok(false))
                })
        });

        if matching.await?? {
            return Ok(true);
        }
    }

    Ok(false)
}
