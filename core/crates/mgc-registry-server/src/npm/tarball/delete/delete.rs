//! Delete a package version through npm unpublish routes.
//! Xóa version package qua route npm unpublish.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

pub(in crate::npm) async fn delete_package_version_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((scope, name, filename)): Path<(String, String, String)>,
) -> Result<StatusCode, StatusCode> {
    delete_package_version_route(
        State(state),
        headers,
        peer,
        Path((scoped_full(&scope, &name), filename)),
    )
    .await
}

pub(in crate::npm) async fn delete_package_version_route(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((name, filename)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    auth.authorize_npm_delete_from(&headers, &name, crate::peer_address(peer))
        .await?;
    // filename: :name-:version.tgz → trích version
    let version = filename
        .strip_suffix(".tgz")
        .and_then(|s| s.strip_prefix(&format!("{}-", name)));
    let Some(version) = version else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let res = store
        .delete_package_version(&name, version)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .then_some(StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)?;
    let _ = store.audit("delete", &name, Some(version), None).await;
    Ok(res)
}
