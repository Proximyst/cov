use crate::health::Component;
use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get_with},
    openapi::OpenApi,
    scalar::Scalar,
    transform::TransformOperation,
};
use axum::{
    Json,
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    middleware,
    response::IntoResponse,
    routing::get,
};
use chrono::Utc;
use metrics::counter;
use proto::health;
use std::net::SocketAddr;
use tokio::sync::{mpsc::Sender, watch};
use tracing::{error, trace};

async fn serve_health(
    State(health): State<watch::Receiver<health::Health>>,
) -> impl IntoApiResponse {
    counter!("http_calls", "endpoint" => "health").increment(1);
    let health = health.borrow().clone();
    let unhealthy = health
        .components
        .values()
        .any(|v| !matches!(v, health::State::Healthy));
    let status = if unhealthy {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    };
    trace!(?status, ?health, "returning health to caller");
    (status, Json(health))
}

fn transform_health(t: TransformOperation) -> TransformOperation {
    let healthy = Component::all_with(Utc::now(), health::State::Healthy);
    let mut unhealthy = Component::all_unknown(Utc::now());
    unhealthy
        .components
        .insert(Component::Database.name().into(), health::State::Healthy);
    unhealthy.components.insert(
        Component::HealthApiActor.name().into(),
        health::State::Unhealthy(String::from("example text")),
    );

    t.description(
        "Fetches the current health of the system. This does not force a re-check of health.",
    )
    .response_with::<200, Json<health::Health>, _>(|r| {
        r.description("The system is healthy.").example(healthy)
    })
    .response_with::<500, Json<health::Health>, _>(|r| {
        r.description("The system is unhealthy.").example(unhealthy)
    })
}

#[derive(Clone)]
struct OpenApiJson(Bytes);

async fn serve_openapi(State(api): State<OpenApiJson>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        api.0,
    )
}

pub(super) async fn health_api_actor(
    addr: SocketAddr,
    health: Sender<(Component, health::State)>,
    current: watch::Receiver<health::Health>,
) {
    let mut api = OpenApi::default();

    let router = ApiRouter::new()
        .route("/scalar", Scalar::new("/api.json").axum_route())
        .route(
            "/api.json",
            get(serve_openapi).layer(middleware::from_fn(super::require_accept_json)),
        )
        .api_route(
            "/health",
            get_with(serve_health, transform_health)
                .with_state(current)
                .layer(middleware::from_fn(super::require_accept_json)),
        );
    let router = router.finish_api_with(&mut api, |t| {
        t.title("cov - Health API")
            .description("The health API for cov.")
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
    trace!(?addr, "bound TcpListener");

    let _ = health
        .send((Component::HealthApiActor, health::State::Healthy))
        .await;
    trace!("sent Healthy for HealthApiActor");

    if let Err(err) = axum::serve(listener, router).await {
        error!(?addr, ?err, "failed to serve axum");
    }
}
