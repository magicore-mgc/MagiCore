// Request builder owner — apply the selected proxy when present.
// Owner builder request — gắn proxy đã chọn nếu có.

use crate::proxy::ProxyConfig;
use anyhow::Result;

impl ProxyConfig {
    /// Apply the selected proxy to a reqwest client builder.
    /// Áp dụng proxy đã chọn vào reqwest client builder.
    pub fn apply(
        &self,
        builder: reqwest::ClientBuilder,
        url: &str,
    ) -> Result<reqwest::ClientBuilder> {
        if let Some(proxy) = self.build_proxy(url)? {
            Ok(builder.proxy(proxy))
        } else {
            Ok(builder)
        }
    }
}

#[cfg(test)]
#[path = "test/apply.rs"]
mod tests;
