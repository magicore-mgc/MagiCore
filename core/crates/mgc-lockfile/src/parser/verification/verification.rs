//! Verifies the bounded lock bytes against their sidecar and signer identity.
//! Xác minh byte lock có giới hạn với sidecar và identity signer.

use std::path::Path;

use mgc_crypto::blake3_signer::Blake3Hasher;
use mgc_crypto::ed25519_signer::{Ed25519PublicKey, Ed25519Signature, verify_signature};

use crate::{Lockfile, LockfileError, LockfileResult, SignatureFile};

use super::error::VerificationFlowError;

/// Load and verify a legacy lockfile signature sidecar.
/// Nạp và xác minh sidecar chữ ký của lockfile legacy.
pub fn load_and_verify_lockfile(
    lockfile_path: &Path,
    signature_path: &Path,
) -> LockfileResult<Lockfile> {
    let lockfile_bytes = super::super::bounded_reader::read_lockfile_bytes(lockfile_path)?;
    let lockfile_text = std::str::from_utf8(&lockfile_bytes)
        .map_err(|error| LockfileError::ParseError(format!("invalid UTF-8: {error}")))?;
    let lockfile = super::super::load::require_legacy_document(
        super::super::parser::parse_document(lockfile_text)?,
    )?;

    let signature_bytes = super::super::bounded_reader::read_signature_file_bytes(signature_path)?;
    let signature_text = std::str::from_utf8(&signature_bytes)
        .map_err(|error| LockfileError::ParseError(format!("invalid signature UTF-8: {error}")))?;
    let signature_file: SignatureFile = signature_text
        .parse()
        .map_err(LockfileError::InvalidSignatureFile)?;

    let current_hash = Blake3Hasher::hash_bytes(&lockfile_bytes);
    let current_hash_text = format!("blake3-{}", current_hash.to_base64());
    verify_hash(&signature_file.lockfile_hash, &current_hash_text)?;

    let signer = lockfile
        .metadata
        .signer
        .as_ref()
        .ok_or(VerificationFlowError::MissingSigner)?;
    let public_key = Ed25519PublicKey::from_base64(&signer.public_key)?;
    let public_key_hash = Blake3Hasher::hash_bytes(&public_key.0);
    let actual_key_id = hex::encode(&public_key_hash.0[..8]);
    if signer.key_id != actual_key_id || signature_file.key_id != actual_key_id {
        return Err(VerificationFlowError::KeyIdMismatch.into());
    }

    let signature_base64 = signature_file
        .signature
        .strip_prefix("ed25519-")
        .unwrap_or(&signature_file.signature);
    let signature = Ed25519Signature::from_base64(signature_base64)?;
    verify_signature(&public_key, current_hash.0.as_ref(), &signature)
        .map_err(|error| VerificationFlowError::SignatureCheck(error.to_string()))?;
    Ok(lockfile)
}

fn verify_hash(expected: &str, actual: &str) -> Result<(), VerificationFlowError> {
    if expected == actual {
        return Ok(());
    }
    Err(VerificationFlowError::HashMismatch {
        expected: expected.to_string(),
        actual: actual.to_string(),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
