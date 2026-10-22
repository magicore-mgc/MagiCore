// Proxy selection tests — cover misses and invalid targets without network.
// Test chọn proxy — kiểm tra miss và URL sai mà không gọi mạng.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::*;
use std::collections::HashMap;

fn config() -> ProxyConfig {
    ProxyConfig {
        http: None,
        https: None,
        no_proxy: Vec::new(),
        per_registry: HashMap::new(),
    }
}

#[test]
fn no_configured_proxy_is_a_miss() {
    assert!(
        config()
            .build_proxy("https://example.test")
            .unwrap()
            .is_none()
    );
}

#[test]
fn invalid_target_returns_an_error() {
    assert!(config().build_proxy("not-a-url").is_err());
}
