// Proxy error tests — keep top-level messages and source chains.
// Test lỗi proxy — giữ thông điệp chính và chuỗi nguyên nhân.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::ProxyError;
use std::error::Error;

#[test]
fn invalid_proxy_message_keeps_legacy_prefix_and_source() {
    let source =
        reqwest::Proxy::all("not a proxy").expect_err("invalid proxy URL must be rejected");
    let error = ProxyError::InvalidProxyUrl(source);

    assert!(error.to_string().starts_with("invalid proxy url: "));
    assert!(error.source().is_some());
}

#[test]
fn invalid_target_error_keeps_source_display() {
    let source = url::Url::parse("http://[::1").expect_err("invalid URL must be rejected");
    let expected = source.to_string();
    let error = ProxyError::InvalidTargetUrl(source);

    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_some());
}
