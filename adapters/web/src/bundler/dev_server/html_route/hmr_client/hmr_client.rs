//! Serve the embedded HMR client script with its JavaScript content type.
//! Phục vụ HMR client nhúng cùng content type JavaScript.

use crate::bundler::hmr::HMR_CLIENT_SCRIPT;
use axum::{
    body::Body,
    http::header,
    response::{IntoResponse, Response},
};

pub(in crate::bundler::dev_server::implementation) async fn serve_hmr_client() -> impl IntoResponse
{
    match Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(Body::from(HMR_CLIENT_SCRIPT))
    {
        Ok(response) => response,
        Err(_) => Response::new(Body::from(HMR_CLIENT_SCRIPT)),
    }
}
