//! Response contract tests for the HMR client endpoint.
//! Test contract response cho endpoint HMR client.

use super::super::implementation::serve_hmr_client;
use crate::bundler::hmr::HMR_CLIENT_SCRIPT;
use axum::{body::to_bytes, http::header, response::IntoResponse};

#[tokio::test]
async fn hmr_client_keeps_javascript_type_and_embedded_script() {
    let response = serve_hmr_client().await.into_response();
    assert_eq!(
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .expect("HMR content type must be present"),
        "application/javascript"
    );
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("HMR response body must be readable");

    assert_eq!(body.as_ref(), HMR_CLIENT_SCRIPT.as_bytes());
}
