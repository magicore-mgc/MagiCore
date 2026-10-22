//! Set a package distribution tag.
//! Đặt dist-tag cho package.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::npm) struct SetDistTagBody {
    version: String,
}

pub(in crate::npm) async fn set_dist_tag(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((name, tag)): Path<(String, String)>,
    Json(body): Json<SetDistTagBody>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    auth.authorize_npm_write_from(&headers, &name, crate::peer_address(peer))
        .await?;
    let mut pkg = store
        .get_package(&name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !pkg.versions.contains_key(&body.version) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let version = body.version;
    if let Some(binding_generation) = auth.trusted_binding_generation(&headers, &name) {
        let updated = store
            .set_trusted_dist_tag(&name, &tag, &version, binding_generation)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if !updated {
            return Err(StatusCode::FORBIDDEN);
        }
        return Ok(Json(serde_json::json!({"tag": tag, "version": version})));
    }
    pkg.dist_tags.insert(tag.clone(), version.clone());
    store
        .put_package(&pkg)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"tag": tag, "version": version})))
}

pub(in crate::npm) async fn set_dist_tag_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((scope, name, tag)): Path<(String, String, String)>,
    Json(body): Json<SetDistTagBody>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    set_dist_tag(
        State(state),
        headers,
        peer,
        Path((scoped_full(&scope, &name), tag)),
        Json(body),
    )
    .await
}
