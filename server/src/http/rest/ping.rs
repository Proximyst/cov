use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::Json;
use chrono::Utc;
use proto::ping::Pong;

pub async fn serve_ping() -> impl IntoApiResponse {
    Json(Pong {
        processed_timestamp: Utc::now(),
    })
}

pub fn transform_ping(t: TransformOperation) -> TransformOperation {
    let example = Pong {
        processed_timestamp: Utc::now(),
    };

    t.description("Ping the server and get a pong back with when it processed the request.")
        .security_requirement("pong")
        .response_with::<200, Json<Pong>, _>(|r| {
            r.description("A successful pong.").example(example)
        })
}
