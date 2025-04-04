mod health;
mod rest;

use crate::{database::Database, health::Component};
use axum::{
    Json,
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use eyre::{Context, Result};
use metrics_exporter_prometheus::PrometheusHandle;
use proto::{
    error,
    health::{Health, State},
};
use std::net::SocketAddr;
use tokio::{
    sync::{mpsc::Sender, watch},
    task::JoinSet,
};

#[derive(Debug, clap::Args)]
#[command(next_help_heading("Http"))]
pub struct HttpArgs {
    /// Where to bind the REST API's insecure HTTP server.
    #[arg(long, env, default_value = "0.0.0.0:8080")]
    api_addr: SocketAddr,

    /// Where to bind the health API's insecure HTTP server.
    ///
    /// This is not intended to be public. If you wish to expose it anyhow, you must manually map this with a reverse proxy.
    #[arg(long, env, default_value = "0.0.0.0:8081")]
    health_addr: SocketAddr,

    /// Where to proxy the frontend requests.
    #[cfg(feature = "proxy-dev")]
    #[arg(long, env, default_value = "localhost:3000")]
    proxy_addr: String,
}

async fn serve_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(error::Error {
            code: error::ErrorCode::NotFound,
            message: Some(String::from("the requested resource was not found")),
        }),
    )
}

fn valid_accept_json_header(v: Option<&HeaderValue>) -> bool {
    let Some(accept) = v else { return true };
    let Ok(accept) = accept.to_str() else {
        return true;
    };

    // This is not perfect. It accepts e.g. `application/jsonx`, which is not a valid media type, but it's good enough.
    // If someone does provide something like that, we'll just assume they actually want JSON after all...
    accept.contains("application/json")
        || accept.contains("application/*")
        || accept.contains("*/*")
}

async fn require_accept_json(request: Request, next: Next) -> Response {
    let headers = request.headers();
    if !valid_accept_json_header(headers.get(header::ACCEPT)) {
        return (
            StatusCode::NOT_ACCEPTABLE,
            Json(error::Error {
                code: error::ErrorCode::NotAcceptable,
                message: Some(String::from(
                    "the requested resource is not available in the requested format",
                )),
            }),
        )
            .into_response();
    }

    next.run(request).await
}

pub fn spawn_health_actor(
    set: &mut JoinSet<()>,
    args: &HttpArgs,
    health: Sender<(Component, State)>,
    current: watch::Receiver<Health>,
    metrics: PrometheusHandle,
) {
    set.spawn(health::health_api_actor(
        args.health_addr,
        health,
        current,
        metrics,
    ));
}

pub async fn spawn_rest_actor(
    set: &mut JoinSet<()>,
    args: &HttpArgs,
    health: Sender<(Component, State)>,
    database: Database,
) -> Result<()> {
    let actor = rest::rest_api_actor(
        args.api_addr,
        #[cfg(feature = "proxy-dev")]
        args.proxy_addr.clone(),
        health,
        database,
    )
    .await
    .wrap_err("failed to create actor")?;
    set.spawn(actor);
    Ok(())
}
