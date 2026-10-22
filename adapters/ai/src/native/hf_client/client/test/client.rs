#![allow(clippy::unwrap_used)]

// Client configuration and secret-safe Debug contract tests.
// Test cấu hình client và bảo đảm Debug không làm lộ bí mật.
use super::super::*;

#[test]
fn client_builder_sets_endpoint_and_token() {
    let client = HuggingFaceClient::new()
        .with_token("hf_token123".into())
        .with_api_url("https://custom.api".into());
    assert_eq!(client.token, Some("hf_token123".to_string()));
    assert_eq!(client.api_url, "https://custom.api");
}

#[test]
fn client_default_uses_huggingface_api() {
    let client = HuggingFaceClient::default();
    assert_eq!(client.api_url, "https://huggingface.co/api");
    assert_eq!(client.token, None);
}

#[test]
fn debug_never_contains_token_or_api_url() {
    let client = HuggingFaceClient::new()
        .with_token("hf_super_secret_fixture".into())
        .with_api_url("https://user:pass@private.example/secret".into());
    let debug = format!("{client:?}");
    assert!(debug.contains("token_configured"));
    assert!(debug.contains("true"));
    assert!(!debug.contains("hf_super_secret_fixture"));
    assert!(!debug.contains("private.example"));
    assert!(!debug.contains("pass"));
}
