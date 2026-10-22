//! Downloads the selected artifact for a PyPI package.
//! Tải artifact đã chọn cho package PyPI.

use mgc_types::PackageId;
use reqwest::Client;

use super::super::{
    error::PyPiClientError,
    metadata_request::{self, StatusPolicy},
    release_selection,
};

pub(in crate::native::pypi_client) async fn download(
    client: &Client,
    metadata_url: &str,
    package_id: &PackageId,
) -> Result<Vec<u8>, PyPiClientError> {
    let metadata = metadata_request::fetch(client, metadata_url, StatusPolicy::DecodeBody).await?;
    let artifact = release_selection::select(&metadata, package_id)?;

    let response = client
        .get(&artifact.url)
        .send()
        .await
        .map_err(PyPiClientError::PackageRequest)?;
    if !response.status().is_success() {
        return Err(PyPiClientError::PackageStatus(response.status()));
    }
    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(PyPiClientError::PackageBody)
}
