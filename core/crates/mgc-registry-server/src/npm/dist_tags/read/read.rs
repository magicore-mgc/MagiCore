//! Read package distribution tags, including scoped aliases.
//! Đọc dist-tag package, gồm cả alias có scope.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(in crate::npm) struct DistTagQuery {
    tag: Option<String>,
}

pub(in crate::npm) async fn get_dist_tags(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Query(query): Query<DistTagQuery>,
) -> Result<Json<HashMap<String, String>>, StatusCode> {
    auth.authorize_package_read_async(&headers, &name).await?;
    if let Some(pkg) = store
        .get_package(&name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        if let Some(tag) = query.tag {
            if let Some(version) = pkg.dist_tags.get(&tag) {
                let mut result = HashMap::new();
                result.insert(tag, version.clone());
                return Ok(Json(result));
            }
            return Err(StatusCode::NOT_FOUND);
        }
        Ok(Json(pkg.dist_tags))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub(in crate::npm) async fn get_dist_tags_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((scope, name)): Path<(String, String)>,
    Query(query): Query<DistTagQuery>,
) -> Result<Json<HashMap<String, String>>, StatusCode> {
    get_dist_tags(
        State(state),
        headers,
        Path(scoped_full(&scope, &name)),
        Query(query),
    )
    .await
}
