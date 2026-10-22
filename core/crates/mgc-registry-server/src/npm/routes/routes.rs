//! Merge endpoint-family routers in original registration order.
//! Ghép router nhóm endpoint theo thứ tự đăng ký ban đầu.

use super::super::{batch_publish, dist_tags, package, search, tarball, user};
use crate::AppState;
use axum::Router;

pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new()
        .merge(package::routes())
        .merge(tarball::routes())
        .merge(dist_tags::routes())
        .merge(user::routes())
        .merge(search::routes())
        .merge(batch_publish::routes())
}
