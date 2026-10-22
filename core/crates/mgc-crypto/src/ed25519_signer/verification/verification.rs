//! Ed25519 verification operations — Thao tác verify Ed25519.
//! Keep ring's verifier and the existing public error mapping — Giữ verifier ring và ánh xạ lỗi public hiện tại.

use ring::signature::{ED25519, UnparsedPublicKey};

use crate::{
    CryptoResult,
    ed25519_signer::{Ed25519PublicKey, Ed25519Signature, error::Ed25519Error},
};

/// Verify a byte-message signature — Xác minh chữ ký của message byte.
pub fn verify_signature(
    public_key: &Ed25519PublicKey,
    message: &[u8],
    signature: &Ed25519Signature,
) -> CryptoResult<()> {
    let public_key_unparsed = UnparsedPublicKey::new(&ED25519, &public_key.0);
    public_key_unparsed
        .verify(message, &signature.0)
        .map_err(|error| Ed25519Error::VerificationFailed(format!("{error:?}")))?;
    Ok(())
}

/// Verify a UTF-8 string signature — Xác minh chữ ký của chuỗi UTF-8.
pub fn verify_string_signature(
    public_key: &Ed25519PublicKey,
    value: &str,
    signature: &Ed25519Signature,
) -> CryptoResult<()> {
    verify_signature(public_key, value.as_bytes(), signature)
}
