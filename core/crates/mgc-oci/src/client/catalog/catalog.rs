//! OCI repository and tag listing.
//! Liệt kê repository và tag theo OCI.

use anyhow::{Result, bail};

use super::super::error::OciClientError;
use crate::client::OciClient;

impl OciClient {
    /// List repositories (GET /v2/_catalog)
    pub async fn list_repositories(&self) -> Result<Vec<String>> {
        let url = format!("{}/v2/_catalog", self.base_url.trim_end_matches('/'));
        let resp = self.client.get(&url).await?;
        if !resp.status().is_success() {
            bail!(OciClientError::CatalogFailed(resp.status().to_string()));
        }
        let json: serde_json::Value = resp.json().await?;
        Ok(json["repositories"]
            .as_array()
            .map(|array| {
                array
                    .iter()
                    .filter_map(|value| value.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// List tags (GET /v2/<repo>/tags/list)
    pub async fn list_tags(&self, repo: &str) -> Result<Vec<String>> {
        let url = format!(
            "{}/v2/{}/tags/list",
            self.base_url.trim_end_matches('/'),
            repo
        );
        let resp = self.client.get(&url).await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }
        let json: serde_json::Value = resp.json().await?;
        Ok(json["tags"]
            .as_array()
            .map(|array| {
                array
                    .iter()
                    .filter_map(|value| value.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }
}
