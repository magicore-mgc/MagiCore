use super::super::prelude::{
    Bytes, HashMap, HeaderMap, IntoResponse, Json, RegistryStore, Response, StatusCode,
};
use super::super::{manifest_validation::validate_sha256_digest, router::header_value};
use super::error::oci_append_error_status;
use crate::model::OciBlobUploadResponse;

pub(in crate::oci::oci) async fn blob_upload_start(
    store: &RegistryStore,
    name: &str,
    query: &HashMap<String, String>,
) -> Result<Response, StatusCode> {
    // Cross-repo mount: POST /v2/{name}/blobs/uploads/?mount={digest}&from={repo}
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let (Some(digest), Some(from)) = (query.get("mount"), query.get("from"))
        && store
            .mount_oci_blob(from, digest, name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let mut resp = Response::new(axum::body::Body::from("{}"));
        *resp.status_mut() = StatusCode::CREATED;
        resp.headers_mut()
            .insert("docker-content-digest", header_value(digest)?);
        resp.headers_mut().insert(
            axum::http::header::LOCATION,
            header_value(&format!("/v2/{}/blobs/{}", name, digest))?,
        );
        return Ok(resp);
    }

    let uuid = uuid::Uuid::new_v4().to_string();
    store
        .create_oci_upload(name, &uuid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return upload URL (JSON body + Location header — OCI spec yêu cầu header)
    let upload_url = format!("/v2/{}/blobs/uploads/{}", name, uuid);

    let mut resp = Json(OciBlobUploadResponse {
        location: upload_url.clone(),
        range: None,
        docker_upload_uuid: uuid,
    })
    .into_response();
    *resp.status_mut() = StatusCode::ACCEPTED;
    resp.headers_mut()
        .insert(axum::http::header::LOCATION, header_value(&upload_url)?);
    Ok(resp)
}

fn parse_upload_range(headers: &HeaderMap, body_len: usize) -> Result<i64, StatusCode> {
    let value = headers
        .get("content-range")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    let (start, end) = value
        .split_once('-')
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    if start.is_empty()
        || end.is_empty()
        || !start.bytes().all(|byte| byte.is_ascii_digit())
        || !end.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    let start = start
        .parse::<u64>()
        .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)?;
    let end = end
        .parse::<u64>()
        .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)?;
    let declared_len = end
        .checked_sub(start)
        .and_then(|range| range.checked_add(1))
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    if declared_len != body_len as u64 {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    i64::try_from(start).map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)
}

pub(in crate::oci::oci) async fn blob_upload_chunk(
    store: &RegistryStore,
    name: &str,
    uuid: &str,
    headers: &HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let start = parse_upload_range(headers, body.len())?;
    let offset = store
        .append_oci_upload(name, uuid, start, &body)
        .await
        .map_err(oci_append_error_status)?;

    let upload_url = format!("/v2/{}/blobs/uploads/{}", name, uuid);
    let last_byte = offset - 1;
    let mut response = Json(OciBlobUploadResponse {
        location: upload_url.clone(),
        range: Some(format!("0-{last_byte}")),
        docker_upload_uuid: uuid.to_string(),
    })
    .into_response();
    *response.status_mut() = StatusCode::ACCEPTED;
    response
        .headers_mut()
        .insert(axum::http::header::LOCATION, header_value(&upload_url)?);
    response
        .headers_mut()
        .insert("range", header_value(&format!("0-{last_byte}"))?);
    Ok(response)
}

pub(in crate::oci::oci) async fn blob_upload_complete(
    store: &RegistryStore,
    name: &str,
    uuid: &str,
    headers: &HeaderMap,
    query: &HashMap<String, String>,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let expected_digest = query.get("digest").ok_or(StatusCode::BAD_REQUEST)?;
    validate_sha256_digest(expected_digest)?;
    let digest_header = header_value(expected_digest)?;
    let location_header = header_value(&format!("/v2/{name}/blobs/{expected_digest}"))?;
    if let Some(offset) = store.oci_upload_offset(name, uuid).await.map_err(|error| {
        tracing::error!("find OCI upload {name}/{uuid}: {error:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })? {
        if !body.is_empty() {
            let start = parse_upload_range(headers, body.len())?;
            if start != offset {
                return Err(StatusCode::RANGE_NOT_SATISFIABLE);
            }
            store
                .append_oci_upload(name, uuid, start, &body)
                .await
                .map_err(oci_append_error_status)?;
        }
        store
            .finalize_oci_upload(name, uuid, expected_digest)
            .await
            .map_err(|error| {
                tracing::error!("finalize_oci_upload {name}/{uuid}: {error:#}");
                StatusCode::BAD_REQUEST
            })?;
    } else {
        if body.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        if body.len() as u64 > crate::storage::max_oci_blob_bytes() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        store
            .put_oci_blob(name, expected_digest, &body)
            .await
            .map_err(|error| {
                tracing::error!("put_oci_blob {name} {expected_digest}: {error:#}");
                StatusCode::BAD_REQUEST
            })?;
    }

    let mut resp = Response::new(axum::body::Body::from("{}"));
    *resp.status_mut() = StatusCode::CREATED;
    resp.headers_mut()
        .insert("docker-content-digest", digest_header);
    resp.headers_mut()
        .insert(axum::http::header::LOCATION, location_header);
    Ok(resp)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
