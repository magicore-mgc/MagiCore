//! registry metadata.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{
    NpmRegistry, PackageMetadata, is_auth_error, metadata_accept_header, with_retry,
};
use anyhow::Result;

impl NpmRegistry {
    pub async fn fetch_metadata(&self, package: &str) -> Result<PackageMetadata> {
        let (metadata, _) = self.fetch_metadata_with_etag(package).await?;
        Ok(metadata)
    }

    pub async fn fetch_metadata_with_etag(
        &self,
        package: &str,
    ) -> Result<(PackageMetadata, Option<String>)> {
        let chain = self.chain();
        let mut last_err: Option<anyhow::Error> = None;
        let accept = metadata_accept_header(self.age_gate_armed());

        for (url, token) in chain {
            let endpoint = format!("{}/{}", url.trim_end_matches('/'), package);
            let result = with_retry("metadata", package, || async {
                let req = self.metadata_request_for(&url, token.as_deref(), &endpoint)?;
                let resp = req
                    .header("Accept", accept)
                    .send()
                    .await?
                    .error_for_status()?;
                let etag = resp
                    .headers()
                    .get("etag")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_owned);
                let metadata: PackageMetadata = resp.json().await?;
                Ok((metadata, etag))
            })
            .await;

            // Return immediately on success — trả ngay khi thành công.
            if let Ok(ok) = result {
                return Ok(ok);
            }
            let Err(e) = result else {
                unreachable!("checked Ok arm above")
            };
            // Scoped redirect refusal is terminal, never a public fallback.
            // Lỗi ranh giới credential không được làm lộ package qua registry kế tiếp.
            let scoped_redirect_refused = self.scoped_clients.is_some()
                && e.chain().any(|cause| {
                    cause
                        .downcast_ref::<reqwest::Error>()
                        .is_some_and(|error| error.is_redirect())
                });
            if is_auth_error(&e) || scoped_redirect_refused {
                return Err(e);
            }
            last_err = Some(e);
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("no registry configured for metadata fetch")))
    }
}
