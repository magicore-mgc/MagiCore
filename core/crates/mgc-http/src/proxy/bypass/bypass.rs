// Proxy bypass owner — apply the existing NO_PROXY host rules.
// Owner bỏ qua proxy — áp dụng quy tắc host NO_PROXY hiện có.

use crate::proxy::ProxyConfig;
use url::Url;

impl ProxyConfig {
    /// Check whether the target URL matches a bypass entry.
    /// Kiểm tra URL đích có khớp mục bỏ qua proxy hay không.
    pub fn is_bypassed(&self, url: &str) -> bool {
        if let Ok(parsed) = Url::parse(url) {
            let host = parsed.host_str().unwrap_or("");
            for np in &self.no_proxy {
                if np == "*" || host == np || host.ends_with(&format!(".{}", np)) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
#[path = "test/bypass.rs"]
mod tests;
