pub mod post {
    use crate::http::rest::auth::{Credentials, Session};
    use aide::axum::IntoApiResponse;
    use axum::{Form, Json, http::StatusCode};
    use metrics::counter;
    #[cfg(feature = "dev")]
    use tracing::trace;
    use tracing::{debug, warn};

    pub async fn login(
        mut session: Session,
        Form(creds): Form<Credentials>,
    ) -> impl IntoApiResponse {
        use proto::{
            AxumEither::{Left, Right},
            error,
        };

        counter!("cov.http.calls", "endpoint" => "v0/login").increment(1);
        debug!(username = creds.username, "attempting to log into user");
        #[cfg(feature = "dev")]
        trace!(password = creds.password, "password provided");
        let user = match session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                debug!(username = creds.username, "invalid credentials");
                return (
                    StatusCode::UNAUTHORIZED,
                    Left(Json(error::Error {
                        code: error::ErrorCode::Unauthorized,
                        message: Some(String::from("invalid credentials")),
                    })),
                );
            }
            Err(err) => {
                warn!(
                    ?err,
                    username = creds.username,
                    "failed to authenticate user credentials"
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Left(Json(error::Error {
                        code: error::ErrorCode::Unauthorized,
                        message: Some(String::from("internal error on login")),
                    })),
                );
            }
        };

        if session.login(&user).await.is_err() {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Left(Json(error::Error {
                    code: error::ErrorCode::Unauthorized,
                    message: Some(String::from("internal error on login")),
                })),
            )
        } else {
            (StatusCode::NO_CONTENT, Right(()))
        }
    }
}
