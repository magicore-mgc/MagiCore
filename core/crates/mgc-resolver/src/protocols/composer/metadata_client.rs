//! Fetch Packagist metadata and dist archives through the shared HTTP client.
//! Tải metadata Packagist và archive dist qua HTTP client dùng chung.

use super::ComposerProtocol;
use super::package_catalog::{PackagistDocument, project_entry, validate_composer_package_name};
use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult};

impl ComposerProtocol {
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

    /// Fetch and parse the p2 document for one `vendor/package`.
    /// Tải và parse tài liệu p2 của một `vendor/package`.
    pub(in crate::protocols::composer) async fn package_doc(
        &self,
        name: &str,
    ) -> MgResult<PackagistDocument> {
        let name = validate_composer_package_name(name)?;
        let url = self.metadata_url(&name);
        let body = self.get_text(&url).await?;
        serde_json::from_str(&body)
            .map_err(|error| MgError::Other(format!("parse Packagist p2 json failed: {error}")))
    }

    /// Re-fetch the p2 document and project one exact version (used for
    /// development dependencies whose selection belongs to the workspace).
    /// Tải lại p2 và chiếu một version chính xác (dùng cho dev dependency,
    /// lựa chọn thuộc workspace).
    pub(in crate::protocols::composer) async fn resolve_exact(
        &self,
        name: &str,
        version: &str,
    ) -> MgResult<ResolvedEntry> {
        let name = validate_composer_package_name(name)?;
        let doc = self.package_doc(&name).await?;
        let target = version.trim().trim_start_matches('v').to_ascii_lowercase();
        let candidate = doc
            .packages
            .get(&name)
            .into_iter()
            .flatten()
            .find(|entry| entry.version.to_ascii_lowercase() == target)
            .ok_or_else(|| {
                MgError::Other(format!(
                    "Packagist metadata for '{name}' has no version {version}"
                ))
            })?;
        project_entry(&name, &candidate.version, candidate, &mut Vec::new())
    }

    /// Download the dist archive of a resolved entry.
    /// Tải archive dist của entry đã resolve.
    pub(in crate::protocols::composer) async fn download_entry(
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
