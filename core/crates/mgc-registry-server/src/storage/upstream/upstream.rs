//! Owns the HTTP client and bounded registry-upstream response retrieval methods.
//! Sở hữu HTTP client và phương thức lấy response từ registry upstream có giới hạn.

use super::super::prelude::*;

/// Upstream proxy (ITEM 4): GET miss → fetch từ registry upstream → cache local.
pub struct Upstream {
    pub(in crate::storage) base: String,
    client: reqwest::Client,
}

impl Upstream {
    /// Request timeout for upstream fetches (registry metadata + tarballs).
    /// Timeout cho request upstream (metadata + tarball).
    const REQUEST_TIMEOUT_SECS: u64 = 30;

    /// Hard cap on a single upstream response body (64 MiB) — a hostile or
    /// misbehaving upstream cannot balloon registry memory.
    /// Giới hạn cứng body 1 response upstream (64 MiB) — upstream xấu không
    /// thể phình memory của registry.
    const MAX_RESPONSE_BYTES: usize = 64 * 1024 * 1024;

    pub fn new(base: String) -> Self {
        Self {
            base: base.trim_end_matches('/').to_string(),
            client: reqwest::Client::builder()
                .user_agent("magicore-registry/0.1")
                .timeout(std::time::Duration::from_secs(Self::REQUEST_TIMEOUT_SECS))
                .connect_timeout(std::time::Duration::from_secs(Self::REQUEST_TIMEOUT_SECS))
                // No redirects: the SSRF same-host check validates the FIRST
                // URL only; a redirect could bounce to an attacker host.
                // Không redirect: check cùng-host SSRF chỉ áp URL đầu tiên;
                // redirect có thể nhảy sang host của kẻ tấn công.
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap_or_default(),
        }
    }

    pub(in crate::storage) async fn fetch_json(
        &self,
        name: &str,
    ) -> Result<Option<serde_json::Value>> {
        let url = format!("{}/{}", self.base, name);
        let resp = self.client.get(&url).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            return Ok(None);
        }
        let body = resp.bytes().await?;
        if body.len() > Self::MAX_RESPONSE_BYTES {
            anyhow::bail!("upstream response exceeds size limit");
        }
        Ok(Some(serde_json::from_slice(&body)?))
    }

    pub(in crate::storage) async fn fetch_bytes(&self, url: &str) -> Result<Option<Vec<u8>>> {
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Ok(None);
        }
        let body = resp.bytes().await?;
        if body.len() > Self::MAX_RESPONSE_BYTES {
            anyhow::bail!("upstream response exceeds size limit");
        }
        Ok(Some(body.to_vec()))
    }
}
