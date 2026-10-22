// ProxyConfig data owner — store proxy environment and registry overrides.
// Owner dữ liệu ProxyConfig — lưu proxy môi trường và ghi đè theo registry.

use std::collections::HashMap;

/// Proxy configuration from environment variables and registry overrides.
/// Cấu hình proxy từ biến môi trường và ghi đè riêng theo registry.
#[derive(Debug, Clone, Default)]
pub struct ProxyConfig {
    pub http: Option<String>,
    pub https: Option<String>,
    pub no_proxy: Vec<String>,
    pub per_registry: HashMap<String, String>, // Registry URL maps to proxy URL — registry URL ánh xạ sang proxy URL.
}

#[cfg(test)]
#[path = "test/proxy_config.rs"]
mod tests;
