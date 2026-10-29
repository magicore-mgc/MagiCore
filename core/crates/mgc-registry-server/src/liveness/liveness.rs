//! Registry liveness probe — probe sống của registry.
//!
//! Three user-facing errors ask "is the registry running?" (`mgc model pull`,
//! model push, and the shared model error). Without a probe endpoint the only way
//! to answer is to try the failing operation again, so this module serves the npm
//! convention clients already probe: `GET /-/ping` → `{}`.
//!
//! It is deliberately unauthenticated and body-free: liveness must not depend on
//! credentials, and it must not reveal registry content.
//! (Ba thông điệp lỗi hỏi "registry có chạy không?" mà không có endpoint để kiểm.
//! Module này phục vụ chuẩn npm `GET /-/ping` → `{}`. Không auth, không nội dung —
//! liveness không được phụ thuộc credential và không được lộ dữ liệu.)

use axum::{Json, Router, routing::get};

/// `GET /-/ping` — the npm-convention liveness probe.
/// `GET /-/ping` — probe sống theo chuẩn npm.
async fn ping() -> Json<serde_json::Value> {
    Json(serde_json::json!({}))
}

/// The ping router carries the same state type as the rest of the app so it can
/// be merged before `with_state`; the handler ignores the state.
/// (Router ping mang cùng kiểu state để merge trước `with_state`; handler bỏ qua
/// state.)
pub(crate) fn routes() -> Router<crate::AppState> {
    Router::new().route("/-/ping", get(ping))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
