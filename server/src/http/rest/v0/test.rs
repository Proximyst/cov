pub mod get {
    use crate::http::rest::auth::Session;
    use axum::response::IntoResponse;
    use metrics::counter;

    // xh --session /tmp/session POST localhost:8080/api/v0/login --form username=admin password=admin
    // xh --session /tmp/session GET localhost:8080/api/v0/test
    // > Some(User { id: 0195d7bf-4515-7b51-8e4e-9a99faa4f6c5, username: "admin", password: "[redacted]" })
    pub async fn test(session: Session) -> impl IntoResponse {
        counter!("cov.http.calls", "endpoint" => "test").increment(1);
        format!("{:?}", session.user)
    }
}
