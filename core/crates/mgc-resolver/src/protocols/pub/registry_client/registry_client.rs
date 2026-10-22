//! Fetch package metadata and archives through the shared MGC HTTP client.
//! Tải metadata và archive qua HTTP client dùng chung của MGC.

use super::super::{
    PubProtocol,
    package_catalog::{PubPackage, validate_pub_package_name},
};
use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult};

impl PubProtocol {
    async fn get_text(&self, url: &str) -> MgResult<String> {
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {url} returned {status}: {body}"
            )));
        }
        Ok(body)
    }

    pub(in crate::protocols::pubdev) async fn package_doc(
        &self,
        name: &str,
    ) -> MgResult<PubPackage> {
        validate_pub_package_name(name)?;
        let url = format!("{}/api/packages/{name}", self.api_url);
        let body = self.get_text(&url).await?;
        serde_json::from_str(&body)
            .map_err(|error| MgError::Other(format!("parse pub.dev json failed: {error}")))
    }

    pub(in crate::protocols::pubdev) async fn download_entry(
        &self,
        entry: &ResolvedEntry,
    ) -> MgResult<Vec<u8>> {
        let resp = self
            .client
            .get(&entry.artifact_url)
            .await
            .map_err(|e| MgError::Network(format!("GET {} failed: {e}", entry.artifact_url)))?;
        let status = resp.status();
        let bytes = resp.bytes().await.map_err(|e| {
            MgError::Network(format!("read {} body failed: {e}", entry.artifact_url))
        })?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {} returned {status}",
                entry.artifact_url
            )));
        }
        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
