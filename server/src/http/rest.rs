mod auth;
#[cfg(not(feature = "dev"))]
mod frontend;
mod get_user;
mod ping;
mod test;

use crate::health::Component;
use aide::{
    axum::{
        ApiRouter,
        routing::{get_with, post},
    },
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
use eyre::{Context, Result};
use metrics::{counter, gauge};
use proto::health;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tracing::error;

#[derive(Clone)]
struct OpenApiJson(Bytes);

async fn serve_openapi(State(api): State<OpenApiJson>) -> impl IntoResponse {
    counter!("cov.http.rest.calls", "endpoint" => "api.json").increment(1);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        api.0,
    )
}

pub(super) async fn rest_api_actor(
    addr: SocketAddr,
    #[cfg(feature = "dev")] proxy_addr: String,
    health: Sender<(Component, health::State)>,
) -> Result<impl Future<Output = ()>> {
    let mut api = OpenApi::default();

    let api_router = ApiRouter::new()
        .api_route(
            "/api/ping",
            get_with(ping::serve_ping, ping::transform_ping),
        )
        .fallback(super::serve_404)
        .layer(middleware::from_fn(super::require_accept_json));

    #[cfg(feature = "dev")]
    let fallback = axum_proxy::builder_http(proxy_addr)
        .wrap_err("failed to create http proxy")?
        .build(axum_proxy::rewrite::Identity);
    #[cfg(not(feature = "dev"))]
    let fallback = get(frontend::serve_frontend);

    let router = ApiRouter::new()
        .route("/api/scalar", Scalar::new("/api/api.json").axum_route())
        .route(
            "/api/api.json",
            get(serve_openapi).layer(middleware::from_fn(super::require_accept_json)),
        )
        .api_route("/api/test", post(test::serve_test)) // TODO: Remove this
        .nest_api_service("/v0", api_router)
        .route_service("/", fallback.clone())
        .route_service("/{*path}", fallback);
    let router = router.finish_api_with(&mut api, |t| {
        t.title("cov - REST API")
            .description("The REST API for cov.")
            .version(env!("CARGO_PKG_VERSION"))
    });

    // We pre-calculate the body for the api.json endpoint.
    // This makes it not have to be calculated on every request to render the OpenAPI.
    // It is quite small, so this is not a big deal.
    let start = std::time::Instant::now();
    let json = serde_json::to_vec(&api).wrap_err("failed to serialize OpenAPI")?;
    let end = std::time::Instant::now();
    gauge!("cov.http.health.openapi", "metric" => "size", "unit" => "bytes").set(json.len() as f64);
    gauge!("cov.http.health.openapi", "metric" => "serialize", "unit" => "nanos")
        .set((end - start).as_nanos() as f64);
    let router = router.with_state(OpenApiJson(Bytes::copy_from_slice(&json)));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .wrap_err("failed to bind to address")?;

    let _ = health
        .send((Component::RestApiActor, health::State::Healthy))
        .await;

    Ok(async {
        if let Err(err) = axum::serve(listener, router).await {
            error!(?err, "failed to serve axum");
        }
    })
}
