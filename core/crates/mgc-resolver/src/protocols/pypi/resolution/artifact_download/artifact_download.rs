//! Download bytes for a resolved PyPI artifact.
//! Tải byte của PyPI artifact đã resolve.

use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult};

pub(in crate::protocols::pypi::resolution) async fn download(
    client: &mgc_http::HttpClient,
    entry: &ResolvedEntry,
) -> MgResult<Vec<u8>> {
    let response = client
        .get(&entry.artifact_url)
        .await
        .map_err(|error| MgError::Network(format!("GET {} failed: {error}", entry.artifact_url)))?;
    let status = response.status();
    let bytes = response.bytes().await.map_err(|error| {
        MgError::Network(format!("read {} body failed: {error}", entry.artifact_url))
    })?;
    if !status.is_success() {
        return Err(MgError::Network(format!(
            "GET {} returned {status}",
            entry.artifact_url
        )));
    }
    Ok(bytes.to_vec())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
