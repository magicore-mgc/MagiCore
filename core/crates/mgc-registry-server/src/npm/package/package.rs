//! Register package metadata read and publish routes.
//! Đăng ký route đọc và publish metadata package.

use super::{get_package, get_package_scoped, publish_package, publish_package_scoped};
use crate::AppState;
use axum::{Router, routing::get};

pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new()
        .route("/npm/{name}", get(get_package).put(publish_package))
        .route("/{name}", get(get_package).put(publish_package))
        .route(
            "/{scope}/{name}",
            get(get_package_scoped).put(publish_package_scoped),
        )
}
