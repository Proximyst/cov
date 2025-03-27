use crate::{database::Database, http::require_accept_json};
use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    middleware,
    routing::{get, post},
};
use eyre::Result;

mod auth;
mod ping;
mod test;

pub async fn router(_db: Database) -> Result<ApiRouter> {
    let router = ApiRouter::new()
        .api_route("/ping", get_with(ping::get::ping, ping::get::transform))
        .route("/login", post(auth::post::login))
        .route("/test", get(test::get::test))
        .layer(middleware::from_fn(require_accept_json));

    Ok(router)
}
