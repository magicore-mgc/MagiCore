//! Preserve the public npm route entrypoint.
//! Giữ ổn định entrypoint route npm công khai.

use crate::AppState;
use axum::Router;

/// npm API routes — alias không prefix (chuẩn npm client: PUT /:name) + /npm/ (nội bộ)
/// Route table is assembled by endpoint family.
/// (Bảng route được ghép theo nhóm endpoint.)
pub fn routes() -> Router<AppState> {
    super::routes::routes()
}
