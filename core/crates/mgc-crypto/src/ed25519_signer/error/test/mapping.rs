//! Local-to-public error mapping tests — Test ánh xạ lỗi local sang public.
//! Keep the compatibility error surface reviewable — Giữ bề mặt lỗi tương thích dễ rà soát.

use crate::{CryptoError, ed25519_signer::error::Ed25519Error};

#[test]
fn maps_typed_length_failures_to_legacy_messages() {
    let signature: CryptoError = Ed25519Error::InvalidSignatureLength(63).into();
    let public_key: CryptoError = Ed25519Error::InvalidPublicKeyLength(31).into();

    assert!(
        matches!(signature, CryptoError::InvalidSignature(message) if message == "expected 64 bytes, got 63")
    );
    assert!(
        matches!(public_key, CryptoError::InvalidPublicKey(message) if message == "expected 32 bytes, got 31")
    );
}

#[test]
fn preserves_base64_source_error() {
    let source = ::base64::Engine::decode(&::base64::engine::general_purpose::STANDARD, "!")
        .expect_err("invalid Base64 must fail");
    let error: CryptoError = Ed25519Error::Base64Decode(source).into();

    assert!(matches!(error, CryptoError::Base64Error(_)));
    assert!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<::base64::DecodeError>())
            .is_some()
    );
}
