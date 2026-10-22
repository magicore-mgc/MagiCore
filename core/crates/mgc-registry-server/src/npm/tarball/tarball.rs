//! Register tarball download, upload and deletion routes.
//! Đăng ký route tải xuống, tải lên và xóa tarball.

use super::{
    delete_package_version_route, delete_package_version_scoped, download_tarball,
    download_tarball_scoped, upload_tarball, upload_tarball_scoped,
};
use crate::AppState;
use axum::{
    Router,
    routing::{get, put},
};

pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/npm/{name}/-/{filename}",
            get(download_tarball).delete(delete_package_version_route),
        )
        .route("/npm/{name}/-/{filename}", put(upload_tarball))
        .route(
            "/{name}/-/{filename}",
            get(download_tarball)
                .put(upload_tarball)
                .delete(delete_package_version_route),
        )
        .route(
            "/{scope}/{name}/-/{filename}",
            get(download_tarball_scoped).delete(delete_package_version_scoped),
        )
        .route("/{scope}/{name}/-/{filename}", put(upload_tarball_scoped))
}
