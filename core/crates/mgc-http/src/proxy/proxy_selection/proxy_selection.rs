// Proxy selection owner — choose the applicable proxy for a URL.
// Owner chọn proxy — chọn proxy phù hợp với URL đích.

use crate::proxy::{ProxyConfig, error::ProxyError};
use anyhow::Result;
use reqwest::Proxy;
use url::Url;

impl ProxyConfig {
    /// Build the reqwest proxy selected for the target URL.
    /// Tạo proxy reqwest được chọn cho URL đích.
    pub fn build_proxy(&self, url: &str) -> Result<Option<Proxy>> {
        if self.is_bypassed(url) {
            return Ok(None);
        }

        let parsed = Url::parse(url).map_err(ProxyError::InvalidTargetUrl)?;
        let scheme = parsed.scheme();
        let proxy_url = if scheme == "https" {
            self.https.as_ref().or(self.http.as_ref())
        } else {
            self.http.as_ref()
        };

        // Per-registry override takes precedence — ghi đè theo registry
        // được ưu tiên trước proxy chung.
        for (reg, proxy) in &self.per_registry {
            if url.starts_with(reg) {
                return Ok(Some(
                    Proxy::all(proxy).map_err(ProxyError::InvalidProxyUrl)?,
                ));
            }
        }

        if let Some(p) = proxy_url {
            Ok(Some(Proxy::all(p).map_err(ProxyError::InvalidProxyUrl)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
#[path = "test/proxy_selection.rs"]
mod tests;
