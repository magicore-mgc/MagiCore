//! Search packages through the npm-compatible API.
//! Tìm package qua API tương thích npm.

use crate::{AppState, model::SearchResult};
use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    size: Option<u32>,
    from: Option<u32>,
}

async fn search(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResult>, StatusCode> {
    auth.authorize_global_read_async(&headers).await?;
    let limit = query.size.unwrap_or(20).min(100);
    let offset = query.from.unwrap_or(0);
    let results = store
        .search_packages(&query.q, limit, offset)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total = results.len() as u64;

    Ok(Json(SearchResult {
        objects: results,
        total,
        time: "0ms".to_string(),
    }))
}
pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new().route("/-/v1/search", get(search))
}
