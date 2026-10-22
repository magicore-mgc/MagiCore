// Registry override tests — verify proxy selection locally without network.
// Test ghi đè registry — xác minh chọn proxy cục bộ, không gọi mạng.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::*;
use std::collections::HashMap;

#[test]
fn registry_override_builds_proxy_before_global_fallback() {
    let config = ProxyConfig {
        http: None,
        https: None,
        no_proxy: Vec::new(),
        per_registry: HashMap::from([(
            "https://registry.example.test".to_string(),
            "http://proxy.example.test:4315".to_string(),
        )]),
    };

    assert!(
        config
            .build_proxy("https://registry.example.test/packages")
            .unwrap()
            .is_some()
    );
}
