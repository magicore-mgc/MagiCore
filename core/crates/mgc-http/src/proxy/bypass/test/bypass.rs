// Proxy bypass tests — preserve the current host matching behavior.
// Test bỏ qua proxy — giữ cách so khớp host hiện có.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::field_reassign_with_default)]

use super::*;
use crate::proxy::test_support::ProxyEnvGuard;

#[test]

fn bypass_localhost() {
    let _guard = ProxyEnvGuard::new();
    let cfg = ProxyConfig::from_env();
    assert!(cfg.is_bypassed("http://localhost:4315"));
    assert!(cfg.is_bypassed("http://127.0.0.1:4315"));
}

#[test]

fn no_proxy_wildcard() {
    // Pure struct construction — no env reads, no guard needed.
    // (Dựng struct thuần — không đọc env, không cần guard.)
    let cfg = ProxyConfig {
        no_proxy: vec!["*".into()],
        ..ProxyConfig::default()
    };
    assert!(cfg.is_bypassed("http://example.com"));
}
