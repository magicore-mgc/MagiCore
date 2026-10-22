//! Signer behavior tests — Test hành vi signer.
//! Keep the test deterministic by using a fixed seed — Dùng seed cố định để test xác định.

use crate::ed25519_signer::{Ed25519Signer, verify_signature, verify_string_signature};

#[test]
fn seeded_signer_emits_verifiable_values() {
    let signer = Ed25519Signer::from_seed(&[7u8; 32]).expect("fixed Ed25519 seed is valid");
    let message = b"owner signer test";
    let signature = signer.sign(message);
    let public_key = signer.public_key();

    assert_eq!(signature.0.len(), 64);
    assert_eq!(public_key.0.len(), 32);
    assert!(verify_signature(&public_key, message, &signature).is_ok());

    let text_signature = signer.sign_string("owner signer test");
    assert!(verify_string_signature(&public_key, "owner signer test", &text_signature).is_ok());
}

#[test]
fn malformed_pkcs8_maps_to_signing_error() {
    let error = Ed25519Signer::from_pkcs8(b"invalid pkcs8")
        .err()
        .expect("malformed PKCS#8 must fail");

    assert!(matches!(
        error,
        crate::CryptoError::SigningFailed(message) if message.starts_with("invalid pkcs8:")
    ));
}
