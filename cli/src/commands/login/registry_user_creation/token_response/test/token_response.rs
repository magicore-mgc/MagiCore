//! Test registry token response validation.
//! Kiểm tra xác thực token response registry.
#![allow(clippy::unwrap_used)]

use super::parse_token_response;
use crate::commands::login::registry_user_creation::RegistryUserCreationError;
use crate::commands::login::registry_user_creation::constants::MAX_REGISTRY_AUTH_RESPONSE_BYTES;

#[test]
fn token_response_returns_non_empty_opaque_token() {
    assert_eq!(
        parse_token_response(br#"{"token":"opaque-token"}"#).unwrap(),
        "opaque-token"
    );
}

#[test]
fn token_response_rejects_invalid_json_and_missing_token() {
    assert!(matches!(
        parse_token_response(b"not-json"),
        Err(RegistryUserCreationError::ResponseInvalid)
    ));
    assert!(matches!(
        parse_token_response(br#"{"ok":true}"#),
        Err(RegistryUserCreationError::TokenMissing)
    ));
}

#[test]
fn token_response_rejects_empty_whitespace_and_control_tokens() {
    for value in ["", "with space", "with\nnewline", "with\u{7f}delete"] {
        let body = serde_json::json!({ "token": value }).to_string();
        assert!(matches!(
            parse_token_response(body.as_bytes()),
            Err(RegistryUserCreationError::TokenMissing)
        ));
    }
}

#[test]
fn token_response_rejects_oversized_body_before_decoding() {
    let oversized_body = vec![b' '; MAX_REGISTRY_AUTH_RESPONSE_BYTES + 1];
    assert!(matches!(
        parse_token_response(&oversized_body),
        Err(RegistryUserCreationError::ResponseTooLarge)
    ));
}
