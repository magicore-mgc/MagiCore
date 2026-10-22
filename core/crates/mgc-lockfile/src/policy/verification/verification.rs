//! Recompute v4 payload/signature math without deciding trust.
//! Tính lại payload/chữ ký V4, không quyết định trust.

use crate::canonical::{LockfileV4, payload_digest};
use crate::{LockfileError, LockfileResult};

use super::root_reference::validate_v4_root_references;
use crate::policy::digest::blake3_digest_bytes;
use crate::policy::report::V4VerifyReport;

/// Verify digest + signature math (no trust decision).
/// Xác minh toán digest + chữ ký (không quyết định tin cậy).
pub fn verify_v4_math(lock: &LockfileV4) -> LockfileResult<V4VerifyReport> {
    validate_v4_root_references(lock)?;
    let recomputed = payload_digest(&lock.payload());
    let digest_ok = recomputed == lock.metadata.lockfile_hash;
    let Some(signature) = lock.metadata.signature.as_ref() else {
        return Ok(V4VerifyReport {
            digest_ok,
            signed: false,
            signature_ok: false,
            key_id: None,
        });
    };
    if signature.algorithm != "ed25519" {
        return Err(LockfileError::VerificationFailed(format!(
            "unsupported signature algorithm '{}'",
            signature.algorithm
        )));
    }
    if signature.digest != lock.metadata.lockfile_hash {
        return Err(LockfileError::VerificationFailed(
            "signature digest does not match lockfile_hash".to_string(),
        ));
    }
    let public_key =
        mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(&signature.public_key)?;
    let public_key_hash = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&public_key.0);
    let actual_key_id = hex::encode(&public_key_hash.0[..8]);
    if signature.key_id != actual_key_id {
        return Err(LockfileError::VerificationFailed(
            "signature key ID does not match the public-key fingerprint".to_string(),
        ));
    }
    let raw = signature
        .signature
        .strip_prefix("ed25519-")
        .unwrap_or(&signature.signature);
    let parsed = mgc_crypto::ed25519_signer::Ed25519Signature::from_base64(raw)?;
    let digest_bytes = blake3_digest_bytes(&signature.digest)?;
    let signature_ok =
        mgc_crypto::ed25519_signer::verify_signature(&public_key, &digest_bytes, &parsed).is_ok();
    Ok(V4VerifyReport {
        digest_ok,
        signed: true,
        signature_ok,
        key_id: Some(signature.key_id.clone()),
    })
}
