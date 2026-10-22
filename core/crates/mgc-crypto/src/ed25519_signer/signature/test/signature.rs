//! Signature conversion tests — Test chuyển đổi chữ ký.
//! Preserve round-trip and invalid-width behavior — Giữ round-trip và hành vi sai độ dài.

use crate::{CryptoError, ed25519_signer::Ed25519Signature};

#[test]
fn round_trips_and_rejects_wrong_width() {
    let signature = Ed25519Signature(vec![0x5a; 64]);
    assert_eq!(
        Ed25519Signature::from_base64(&signature.to_base64()).expect("test operation must succeed"),
        signature
    );

    let error = Ed25519Signature::from_base64("AA==").expect_err("test operation must fail");
    assert!(
        matches!(error, CryptoError::InvalidSignature(message) if message == "expected 64 bytes, got 1")
    );
}
