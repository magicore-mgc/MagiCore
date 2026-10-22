//! Fetch and decode one Web registry package-search response.
//! Lấy và giải mã một response tìm package từ Web registry.

use serde_json::Value;

use crate::commands::web_registry_config::{search_endpoint, web_registry_url};

use super::super::{error::SearchError, query::SearchRequest, types::RegistrySearchResponse};

pub(in crate::commands::search) async fn fetch_registry_search(
    request: &SearchRequest,
) -> Result<RegistrySearchResponse, SearchError> {
    let registry_url = web_registry_url()?;
    let url = search_endpoint(
        &registry_url,
        &request.search_query,
        request.size,
        request.from,
    )?;

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(SearchError::Request)?;
    let data: Value = response.json().await.map_err(SearchError::Response)?;

    Ok(decode_search_response(data))
}

fn decode_search_response(data: Value) -> RegistrySearchResponse {
    let total = data["total"].as_u64().unwrap_or(0);
    let objects = data["objects"].as_array().cloned().unwrap_or_default();

    RegistrySearchResponse { total, objects }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
