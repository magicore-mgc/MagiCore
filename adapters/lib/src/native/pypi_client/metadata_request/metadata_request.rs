//! Fetches and decodes the PyPI JSON metadata response.
//! Tải và giải mã response metadata JSON PyPI.

use reqwest::{Client, Response};

use super::super::{error::PyPiClientError, types::PyPiMetadata};

#[derive(Clone, Copy)]
pub(in crate::native::pypi_client) enum StatusPolicy {
    RequireSuccess,
    DecodeBody,
}

pub(in crate::native::pypi_client) async fn fetch(
    client: &Client,
    url: &str,
    status_policy: StatusPolicy,
) -> Result<PyPiMetadata, PyPiClientError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(PyPiClientError::MetadataRequest)?;
    decode_response(response, status_policy).await
}

async fn decode_response(
    response: Response,
    status_policy: StatusPolicy,
) -> Result<PyPiMetadata, PyPiClientError> {
    if matches!(status_policy, StatusPolicy::RequireSuccess) && !response.status().is_success() {
        return Err(PyPiClientError::RegistryStatus(response.status()));
    }
    response
        .json()
        .await
        .map_err(PyPiClientError::MetadataParse)
}
