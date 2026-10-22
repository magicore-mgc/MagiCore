//! Register npm distribution-tag routes.
//! Đăng ký các route dist-tag npm.

use super::{
    delete_dist_tag, delete_dist_tag_scoped, get_dist_tags, get_dist_tags_scoped, set_dist_tag,
    set_dist_tag_scoped,
};
use crate::AppState;
use axum::{
    Router,
    routing::{get, put},
};

pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/-/package/{name}/dist-tags/{tag}",
            put(set_dist_tag).delete(delete_dist_tag),
        )
        .route("/-/package/{name}/dist-tags", get(get_dist_tags))
        .route(
            "/-/package/{scope}/{name}/dist-tags/{tag}",
            put(set_dist_tag_scoped).delete(delete_dist_tag_scoped),
        )
        .route(
            "/-/package/{scope}/{name}/dist-tags",
            get(get_dist_tags_scoped),
        )
}
