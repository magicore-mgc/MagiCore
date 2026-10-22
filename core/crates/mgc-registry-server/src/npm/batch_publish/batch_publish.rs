//! Preserve npm batch-publish endpoint behavior.
//! Giữ hành vi endpoint batch-publish npm.

use crate::AppState;
use axum::{Router, body::Bytes, extract::State, http::StatusCode, response::Json, routing::post};

async fn batch_publish(
    State((_, _)): State<AppState>,
    _body: Bytes,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new().route("/-/v1/publish", post(batch_publish))
}
