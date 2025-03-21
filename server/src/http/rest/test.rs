use aide::axum::IntoApiResponse;
use axum::Json;

pub async fn serve_test(body: String) -> impl IntoApiResponse {
    Json(crate::report::parse_report(&body))
}
