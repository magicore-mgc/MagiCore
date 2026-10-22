//! Remove a package distribution tag.
//! Xóa dist-tag khỏi package.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

pub(in crate::npm) async fn delete_dist_tag(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((name, tag)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    auth.authorize_npm_delete_from(&headers, &name, crate::peer_address(peer))
        .await?;
    let mut pkg = store
        .get_package(&name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if pkg.dist_tags.remove(&tag).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    store
        .put_package(&pkg)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub(in crate::npm) async fn delete_dist_tag_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((scope, name, tag)): Path<(String, String, String)>,
) -> Result<StatusCode, StatusCode> {
    delete_dist_tag(
        State(state),
        headers,
        peer,
        Path((scoped_full(&scope, &name), tag)),
    )
    .await
}
