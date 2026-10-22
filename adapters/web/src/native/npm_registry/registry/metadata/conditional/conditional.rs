//! registry metadata conditional.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{
    NpmRegistry, PackageMetadata, metadata_accept_header, with_retry,
};
use anyhow::Result;

impl NpmRegistry {
    pub async fn fetch_metadata_conditional(
        &self,
        package: &str,
        etag: Option<&str>,
    ) -> Result<Option<(PackageMetadata, String)>> {
        let url = format!("{}/{}", self.registry_url.trim_end_matches('/'), package);
        let etag_owned = etag.map(|s| s.to_string());
        with_retry("metadata-conditional", package, || async {
            let accept = metadata_accept_header(self.age_gate_armed());
            let mut req = self
                .metadata_request_for(&self.registry_url, self.token.as_deref(), &url)?
                .header("Accept", accept);
            if let Some(ref etag_val) = etag_owned {
                req = req.header("If-None-Match", etag_val);
            }
            let resp = req.send().await?;

            if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
                return Ok(None);
            }

            let resp = resp.error_for_status()?;
            let new_etag = resp
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let metadata: PackageMetadata = resp.json().await?;
            Ok(Some((metadata, new_etag)))
        })
        .await
    }
}
