pub mod get {
    use aide::{axum::IntoApiResponse, transform::TransformOperation};
    use axum::Json;
    use chrono::Utc;
    use metrics::counter;
    use proto::ping::Pong;

    pub async fn ping() -> impl IntoApiResponse {
        counter!("cov.http.calls", "endpoint" => "v0/ping").increment(1);
        Json(Pong {
            processed_timestamp: Utc::now(),
        })
    }

    pub fn transform(t: TransformOperation) -> TransformOperation {
        let example = Pong {
            processed_timestamp: Utc::now(),
        };

        t.description("Ping the server and get a pong back with when it processed the request.")
            .security_requirement("pong")
            .response_with::<200, Json<Pong>, _>(|r| {
                r.description("A successful pong.").example(example)
            })
    }
}
