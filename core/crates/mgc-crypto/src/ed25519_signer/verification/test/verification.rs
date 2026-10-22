//! Signature verification tests — Test xác minh chữ ký.
//! Preserve valid, mismatched and malformed-signature outcomes — Giữ kết quả chữ ký hợp lệ, mismatch và sai định dạng.

use crate::{
    CryptoError,
    ed25519_signer::{Ed25519Signer, verify_signature, verify_string_signature},
};

#[test]
fn verifies_bytes_and_strings_and_rejects_tampering() {
    let signer = Ed25519Signer::from_seed(&[8u8; 32]).expect("fixed Ed25519 seed is valid");
    let bytes = b"signed bytes";
    let signature = signer.sign(bytes);
    let public_key = signer.public_key();

    assert!(verify_signature(&public_key, bytes, &signature).is_ok());
    assert!(matches!(
        verify_signature(&public_key, b"changed bytes", &signature),
        Err(CryptoError::VerificationFailed(message)) if message.starts_with("signature invalid:")
    ));

    let text_signature = signer.sign_string("signed text");
    assert!(verify_string_signature(&public_key, "signed text", &text_signature).is_ok());
    assert!(verify_string_signature(&public_key, "changed text", &text_signature).is_err());
}
