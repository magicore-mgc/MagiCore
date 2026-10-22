//! Registers the stable PyPI route patterns.
//! Đăng ký các pattern PyPI ổn định.

use super::super::{
    download_file::download_file, simple_index::simple_index, upload_legacy::upload_legacy,
};
use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

/// PyPI routes — namespace riêng /pypi để không đụng npm routes gốc
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/pypi/simple/{name}", get(simple_index))
        .route("/pypi/simple/{name}/", get(simple_index))
        .route("/pypi/packages/{name}/{filename}", get(download_file))
        .route("/pypi/legacy/", post(upload_legacy))
}
