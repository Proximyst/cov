use axum::{
    http::{HeaderName, StatusCode, Uri, header},
    response::IntoResponse,
};
use include_dir::{Dir, File};
use tracing::trace;

static FRONTEND_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/../web/out");

pub async fn serve_frontend(uri: Uri) -> impl IntoResponse {
    serve_frontend0(uri)
}

fn serve_frontend0(uri: Uri) -> impl IntoResponse {
    let span = tracing::debug_span!("serve_frontend", path = uri.path());
    let _span = span.enter();

    trace!("got frontend request");
    match read_file(uri.path()) {
        Some((content_type, file)) => {
            trace!(
                content_type,
                len = file.contents().len(),
                "found file; returning it"
            );
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                file.contents(),
            )
        }
        None => {
            trace!("could not find file");
            not_found()
        }
    }
}

fn not_found() -> (StatusCode, [(HeaderName, &'static str); 1], &'static [u8]) {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "application/json")],
        br#"{"code":"not-found","message":"the requested resource was not found"}"#,
    )
}

fn read_file(path: &str) -> Option<(&'static str, &'static File<'static>)> {
    let path = match path.trim_start_matches('/') {
        "" => "index.html",
        path if !path.contains('.') => &format!("{}/{path}.html", path.trim_end_matches('/')),
        path => path,
    };

    let file = FRONTEND_DIR.get_file(path)?;

    let content_type = match file.path().extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("css") => "text/css",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    };

    Some((content_type, file))
}

#[cfg(test)]
mod tests {
    #[test]
    fn not_found_body_is_semantically_correct() {
        use proto::error::{Error, ErrorCode};

        // It's easier to deal with the bytes directly than to serialize a JSON error.
        let body = super::not_found().2;
        let body: Error = serde_json::from_slice(body).unwrap();

        let expected = Error {
            code: ErrorCode::NotFound,
            message: Some("the requested resource was not found".to_string()),
        };

        assert_eq!(body, expected);
    }
}
