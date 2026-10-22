//! Shared crypto error display and source tests — Test display và source lỗi crypto dùng chung.
//! Freeze compatibility diagnostics across every error family — Cố định diagnostic tương thích cho mọi nhóm lỗi.

use std::error::Error as _;

use crate::CryptoError;

#[test]
fn preserves_operation_error_text() {
    let cases = [
        (
            CryptoError::Blake3Failed("hash".to_string()),
            "Blake3 hash failed: hash",
        ),
        (
            CryptoError::SigningFailed("sign".to_string()),
            "Ed25519 signing failed: sign",
        ),
        (
            CryptoError::VerificationFailed("verify".to_string()),
            "Ed25519 verification failed: verify",
        ),
        (
            CryptoError::KeyringFailed("key".to_string()),
            "Keyring operation failed: key",
        ),
        (
            CryptoError::InvalidSignature("signature".to_string()),
            "Invalid signature format: signature",
        ),
        (
            CryptoError::InvalidPublicKey("public key".to_string()),
            "Invalid public key format: public key",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn preserves_io_serialization_and_base64_sources() {
    let io = CryptoError::IoError(std::io::Error::other("disk failure"));
    assert_eq!(io.to_string(), "IO error: disk failure");
    assert!(io.source().is_some());

    let serialization_source = serde_json::from_str::<serde_json::Value>("invalid json")
        .expect_err("test operation must fail");
    let serialization = CryptoError::SerializationError(serialization_source);
    assert!(
        serialization
            .to_string()
            .starts_with("Serialization error:")
    );
    assert!(serialization.source().is_some());

    use base64::Engine as _;
    let base64_source = base64::engine::general_purpose::STANDARD
        .decode("!")
        .expect_err("test operation must fail");
    let base64 = CryptoError::Base64Error(base64_source);
    assert!(base64.to_string().starts_with("Base64 decode error:"));
    assert!(base64.source().is_some());
}
