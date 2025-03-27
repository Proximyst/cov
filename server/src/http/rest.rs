mod auth;
mod v0;

#[cfg(not(feature = "dev"))]
mod frontend;

use crate::{database::Database, health::Component};
use aide::{axum::ApiRouter, openapi::OpenApi, scalar::Scalar};
use axum::{
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    middleware,
    response::IntoResponse,
    routing::get,
};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{MemoryStore, SessionManagerLayer},
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
    database: Database,
) -> Result<impl Future<Output = ()>> {
    let v0 = v0::router(database.clone())
        .await
        .wrap_err("failed to create v0 router")?;

    #[cfg(feature = "dev")]
    let fallback = axum_proxy::builder_http(proxy_addr)
        .wrap_err("failed to create http proxy")?
        .build(axum_proxy::rewrite::Identity);
    #[cfg(not(feature = "dev"))]
    let fallback = get(frontend::serve_frontend);

    let session_store = MemoryStore::default(); // TODO: Make a persistent store
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(axum_login::tower_sessions::Expiry::OnInactivity(
            axum_login::tower_sessions::cookie::time::Duration::days(1),
        ));
    let backend = auth::Backend::new(database.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let router = ApiRouter::new()
        .route("/api/scalar", Scalar::new("/api/api.json").axum_route())
        .route(
            "/api/api.json",
            get(serve_openapi).layer(middleware::from_fn(super::require_accept_json)),
        )
        .nest_api_service("/api/v0", v0)
        .route_service("/", fallback.clone())
        .route_service("/{*path}", fallback)
        .layer(auth_layer);

    let mut api = OpenApi::default();
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
