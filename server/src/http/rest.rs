mod ping;

use crate::health::Component;
use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    middleware,
    response::IntoResponse,
    routing::get,
};
use proto::health;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tracing::error;

#[derive(Clone)]
struct OpenApiJson(Bytes);

async fn serve_openapi(State(api): State<OpenApiJson>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        api.0,
    )
}

pub(super) async fn rest_api_actor(addr: SocketAddr, health: Sender<(Component, health::State)>) {
    let mut api = OpenApi::default();

    let api_router = ApiRouter::new()
        .api_route("/ping", get_with(ping::serve_ping, ping::transform_ping))
        .fallback(super::serve_404)
        .layer(middleware::from_fn(super::require_accept_json));

    let router = ApiRouter::new()
        .route("/scalar", Scalar::new("/api.json").axum_route())
        .route(
            "/api.json",
            get(serve_openapi).layer(middleware::from_fn(super::require_accept_json)),
        )
        .nest_api_service("/v0", api_router);
    let router = router.finish_api_with(&mut api, |t| {
        t.title("cov - REST API")
            .description("The REST API for cov.")
            .version(env!("CARGO_PKG_VERSION"))
    });

    // We pre-calculate the body for the api.json endpoint.
    // This makes it not have to be calculated on every request to render the OpenAPI.
    // It is quite small, so this is not a big deal.
    let json = match serde_json::to_vec(&api) {
        Ok(json) => json,
        Err(err) => {
            error!(?err, "failed to serialize OpenAPI");
            return;
        }
    };
    let router = router.with_state(OpenApiJson(Bytes::copy_from_slice(&json)));

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            error!(?addr, ?err, "failed to bind to address");
            return;
        }
    };

    let _ = health
        .send((Component::RestApiActor, health::State::Healthy))
        .await;

    if let Err(err) = axum::serve(listener, router).await {
        error!(?addr, ?err, "failed to serve axum");
    }
}
