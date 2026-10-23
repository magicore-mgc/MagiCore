use super::super::blob_read::{blob_delete, blob_get, blob_head};
use super::super::blob_upload::{blob_upload_chunk, blob_upload_complete, blob_upload_start};
use super::super::catalog::{catalog, tags_list};
use super::super::manifest_routes::{manifest_delete, manifest_get, manifest_put};
use super::super::prelude::{
    AppState, Bytes, HashMap, HeaderMap, HeaderValue, IntoResponse, Json, Method, Path, Query,
    Response, Router, State, StatusCode,
};
use axum::routing::get;

#[path = "route_parse/mod.rs"]
mod route_parse;

use route_parse::split_oci;

pub(in crate::oci::oci) fn header_value(value: &str) -> Result<HeaderValue, StatusCode> {
    HeaderValue::from_str(value).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// OCI routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Blob + manifest operations, dispatcher parses `rest`
        .route(
            "/v2/{*rest}",
            get(v2_dispatch)
                .head(v2_dispatch)
                .post(v2_dispatch)
                .put(v2_dispatch)
                .patch(v2_dispatch)
                .delete(v2_dispatch),
        )
        // Root ping + catalog (static, preferred over the catch-all)
        .route("/v2/", get(v2_root))
        .route("/v2/_catalog", get(catalog))
}

/// Entry point for `/v2/*rest` — dispatches by section + HTTP method.
async fn v2_dispatch(
    state: State<AppState>,
    method: Method,
    Path(rest): Path<String>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let Some((name, section, tail)) = split_oci(&rest) else {
        // Not a known OCI route shape
        return Err(StatusCode::NOT_FOUND);
    };
    let (store, auth) = state.0;
    // Upload sessions also mutate storage — phiên upload cũng ghi dữ liệu.
    let trusted_publisher = if section == "blobs/uploads"
        || !matches!(method, Method::GET | Method::HEAD)
    {
        let package_scope = crate::trusted::scoped_package("oci", name)?;
        let publisher = auth
            .authorize_package_write_from(&headers, &package_scope, crate::peer_address(peer))
            .await?;
        if publisher.is_some()
            && !(section == "blobs/uploads" || (section == "manifests" && method == Method::PUT))
        {
            return Err(StatusCode::FORBIDDEN);
        }
        publisher
    } else {
        let package_scope = crate::trusted::scoped_package("oci", name)?;
        auth.authorize_package_read_async(&headers, &package_scope)
            .await?;
        None
    };

    // Cross-repository mounts also require read access to the source repository.
    // Mount liên repository cũng cần quyền đọc repository nguồn.
    if section == "blobs/uploads"
        && query.contains_key("mount")
        && let Some(from) = query.get("from")
    {
        let source_scope = crate::trusted::scoped_package("oci", from)?;
        auth.authorize_package_read_async(&headers, &source_scope)
            .await?;
    }

    match section {
        "blobs/uploads" => match tail {
            "/" | "" if method == Method::POST => blob_upload_start(&store, name, &query)
                .await
                .map(|r| r.into_response()),
            "/" | "" => Err(StatusCode::METHOD_NOT_ALLOWED),
            uuid_tail => match method {
                Method::PATCH => {
                    blob_upload_chunk(&store, name, &uuid_tail[1..], &headers, body).await
                }
                Method::PUT => {
                    blob_upload_complete(&store, name, &uuid_tail[1..], &headers, &query, body)
                        .await
                }
                _ => Err(StatusCode::METHOD_NOT_ALLOWED),
            },
        },
        "blobs" => match method {
            Method::HEAD => blob_head(&store, name, &tail[1..]).await,
            Method::GET => blob_get(&store, name, &tail[1..]).await,
            Method::DELETE => blob_delete(&store, name, &tail[1..]).await,
            _ => Err(StatusCode::METHOD_NOT_ALLOWED),
        },
        "manifests" => match method {
            Method::GET => manifest_get(&store, name, &tail[1..]).await,
            Method::PUT => {
                manifest_put(
                    &store,
                    name,
                    &tail[1..],
                    body,
                    trusted_publisher,
                    auth.attestation_key(),
                )
                .await
            }
            Method::DELETE => manifest_delete(&store, name, &tail[1..]).await,
            _ => Err(StatusCode::METHOD_NOT_ALLOWED),
        },
        "tags/list" if method == Method::GET => tags_list(&store, name).await,
        _ => Err(StatusCode::METHOD_NOT_ALLOWED),
    }
}

async fn v2_root() -> Json<serde_json::Value> {
    Json(serde_json::json!({}))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
