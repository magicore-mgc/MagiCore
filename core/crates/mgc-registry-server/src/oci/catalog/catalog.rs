use super::super::prelude::{
    AppState, HashMap, HeaderMap, IntoResponse, Json, Query, RegistryStore, Response, State,
    StatusCode,
};

pub(in crate::oci::oci) async fn tags_list(
    store: &RegistryStore,
    name: &str,
) -> Result<Response, StatusCode> {
    let tags = store
        .list_oci_tags(name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({
        "name": name,
        "tags": tags
    }))
    .into_response())
}

pub(in crate::oci::oci) async fn catalog(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    auth.authorize_global_read_async(&headers).await?;
    let repos = store
        .list_oci_repos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({
        "repositories": repos
    })))
}
