//! Pull and push OCI manifests.
//! Tải và đẩy manifest OCI.

use anyhow::{Result, bail};

use super::super::error::OciClientError;
use crate::client::OciClient;
use crate::manifest::OciManifest;

impl OciClient {
    /// Pull manifest
    pub async fn pull_manifest(&self, repo: &str, reference: &str) -> Result<OciManifest> {
        let url = format!(
            "{}/v2/{}/manifests/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            reference
        );

        let resp = self.client.get(&url).await?;
        if !resp.status().is_success() {
            bail!(OciClientError::PullManifestFailed(
                resp.status().to_string()
            ));
        }

        let manifest: OciManifest = resp.json().await?;
        Ok(manifest)
    }

    /// Push manifest
    pub async fn push_manifest(
        &self,
        repo: &str,
        reference: &str,
        manifest: &OciManifest,
    ) -> Result<()> {
        let url = format!(
            "{}/v2/{}/manifests/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            reference
        );

        let body = serde_json::to_vec(manifest)?;

        let resp = self.client.put(&url, body).await?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            bail!(OciClientError::PushManifestFailed {
                status: status.to_string(),
                body: text,
            });
        }
        Ok(())
    }
}
