// Proxy environment owner — read proxy variables with existing precedence.
// Owner môi trường proxy — đọc biến proxy theo thứ tự ưu tiên hiện có.

use crate::proxy::ProxyConfig;
use std::collections::HashMap;
use std::env;

impl ProxyConfig {
    /// Load proxy settings from the process environment.
    /// Đọc cấu hình proxy từ môi trường của tiến trình.
    pub fn from_env() -> Self {
        let http = env::var("HTTP_PROXY")
            .or_else(|_| env::var("http_proxy"))
            .ok();
        let https = env::var("HTTPS_PROXY")
            .or_else(|_| env::var("https_proxy"))
            .ok();
        let no_proxy = env::var("NO_PROXY")
            .or_else(|_| env::var("no_proxy"))
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| vec!["localhost".into(), "127.0.0.1".into()]);

        Self {
            http,
            https,
            no_proxy,
            per_registry: HashMap::new(),
        }
    }
}

#[cfg(test)]
#[path = "test/environment.rs"]
mod tests;
