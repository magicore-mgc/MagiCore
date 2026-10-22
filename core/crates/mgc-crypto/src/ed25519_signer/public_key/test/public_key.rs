//! Public-key conversion tests — Test chuyển đổi public key.
//! Preserve round-trip and invalid-width behavior — Giữ round-trip và hành vi sai độ dài.

use crate::{CryptoError, ed25519_signer::Ed25519PublicKey};

#[test]
fn round_trips_and_rejects_wrong_width() {
    let key = Ed25519PublicKey(vec![0x31; 32]);
    assert_eq!(
        Ed25519PublicKey::from_base64(&key.to_base64()).expect("test operation must succeed"),
        key
    );

    let error = Ed25519PublicKey::from_base64("AA==").expect_err("test operation must fail");
    assert!(
        matches!(error, CryptoError::InvalidPublicKey(message) if message == "expected 32 bytes, got 1")
    );
}
