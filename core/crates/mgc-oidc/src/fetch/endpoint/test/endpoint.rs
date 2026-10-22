//! Secure endpoint URL cases — ca kiểm thử URL endpoint an toàn.
//!
use super::super::secure_url;

#[test]
fn accepts_https_without_credentials_or_fragment() {
    assert!(secure_url("https://issuer.example/token?audience=registry").is_ok());
}

#[test]
fn rejects_http_identity_endpoint() {
    assert!(secure_url("http://issuer.example/token").is_err());
}

#[test]
fn rejects_url_credentials_and_fragments() {
    assert!(secure_url("https://user:secret@issuer.example/token").is_err());
    assert!(secure_url("https://issuer.example/token#fragment").is_err());
}
