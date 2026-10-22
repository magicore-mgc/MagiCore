// Proxy application tests — build a client with and without a selected proxy.
// Test áp dụng proxy — tạo client khi có và không chọn proxy.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn builder_without_proxy_remains_buildable() {
    let builder = reqwest::Client::builder();
    assert!(
        ProxyConfig::default()
            .apply(builder, "https://example.test")
            .is_ok()
    );
}

#[test]
fn builder_with_proxy_remains_buildable() {
    let config = ProxyConfig {
        http: Some("http://proxy.example.test:4315".to_string()),
        ..ProxyConfig::default()
    };
    assert!(
        config
            .apply(reqwest::Client::builder(), "https://example.test")
            .is_ok()
    );
}
