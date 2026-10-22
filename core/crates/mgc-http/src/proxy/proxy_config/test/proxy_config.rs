// ProxyConfig tests — verify the empty default configuration.
// Test ProxyConfig — xác minh cấu hình mặc định rỗng.

#![cfg(test)]

use super::*;

#[test]
fn default_has_no_proxy_or_registry_overrides() {
    let config = ProxyConfig::default();

    assert!(config.http.is_none());
    assert!(config.https.is_none());
    assert!(config.no_proxy.is_empty());
    assert!(config.per_registry.is_empty());
}
