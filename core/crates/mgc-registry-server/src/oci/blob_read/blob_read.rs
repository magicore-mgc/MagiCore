use super::super::prelude::{HeaderValue, IntoResponse, RegistryStore, Response, StatusCode};
use super::super::router::header_value;

pub(in crate::oci::oci) async fn blob_head(
    store: &RegistryStore,
    name: &str,
    digest: &str,
) -> Result<Response, StatusCode> {
    match store.oci_blob_exists(name, digest).await {
        Ok(true) => Ok(StatusCode::OK.into_response()),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(in crate::oci::oci) async fn blob_get(
    store: &RegistryStore,
    name: &str,
    digest: &str,
) -> Result<Response, StatusCode> {
    match store.get_oci_blob(name, digest).await {
        Ok(Some(data)) => {
            let mut resp = Response::new(axum::body::Body::from(data));
            resp.headers_mut().insert(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
            // Add Docker-Content-Digest header
            let digest_header = format!(
                "sha256:{}",
                digest.strip_prefix("sha256:").unwrap_or(digest)
            );
            resp.headers_mut()
                .insert("docker-content-digest", header_value(&digest_header)?);
            Ok(resp.into_response())
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(in crate::oci::oci) async fn blob_delete(
    store: &RegistryStore,
    name: &str,
    digest: &str,
) -> Result<Response, StatusCode> {
    if store
        .delete_oci_blob(name, digest)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
