//! Authentication middleware entry points.
// Challenge 401 cùng middleware auth theo request.

use axum::{http::StatusCode, response::Response};

/// 401 kèm WWW-Authenticate challenge (chuẩn OCI/Docker).
pub fn unauthorized_response() -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(
            "WWW-Authenticate",
            "Basic realm=\"magicore-registry\", Bearer realm=\"magicore-registry\"",
        )
        .body(axum::body::Body::empty())
        .expect("static 401 response")
}
