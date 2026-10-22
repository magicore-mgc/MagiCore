#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Shared HTTP client construction tests.
//! Test tạo HTTP client dùng chung.

use super::*;

#[test]
fn http_client_default_works() {
    let client = HttpClient::new().unwrap();
    assert!(client.retry.delay(0).as_secs() >= 1);
}

#[test]
fn client_build_error_keeps_context_and_source() {
    let source = std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid TLS option");
    let error = anyhow::Error::new(HttpClientError::BuildClient(Box::new(source)));
    assert_eq!(
        error.to_string(),
        "build reqwest client: invalid TLS option"
    );
    assert_eq!(
        error.chain().last().unwrap().to_string(),
        "invalid TLS option"
    );
}
