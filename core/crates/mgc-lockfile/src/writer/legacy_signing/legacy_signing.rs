//! Signs legacy lockfile bytes and publishes the signature sidecar.
//! Ký bytes lock legacy và publish sidecar chữ ký.

use crate::{Lockfile, LockfileError, LockfileResult, SignatureFile, SignerInfo};
use mgc_crypto::blake3_signer::Blake3Hasher;
use mgc_crypto::keyring::{KeyPair, Keyring};
use std::path::Path;

use super::error::LegacySigningError;

/// Sign a legacy lockfile and write its signature sidecar.
/// Ký lockfile legacy và ghi sidecar chữ ký.
pub fn sign_and_write_lockfile(
    lockfile: &mut Lockfile,
    lockfile_path: &Path,
    key_pair: &KeyPair,
) -> LockfileResult<()> {
    crate::writer::ensure_not_v4_legacy_write(lockfile_path)?;
    crate::verifier::verify_ownership_completeness(lockfile)?;
    let signer = key_pair.signer()?;

    lockfile.metadata.signer = Some(SignerInfo {
        key_id: key_pair.key_id.clone(),
        public_key: signer.public_key().to_base64(),
        signed_at: chrono::Utc::now().to_rfc3339(),
    });
    lockfile.metadata.lockfile_hash = String::new();

    crate::writer::write_lockfile_unchecked(lockfile, lockfile_path)?;

    let lockfile_bytes = std::fs::read(lockfile_path)?;
    let lockfile_hash = Blake3Hasher::hash_bytes(&lockfile_bytes);
    let lockfile_hash_str = format!("blake3-{}", lockfile_hash.to_base64());
    let signature = signer.sign(lockfile_hash.0.as_ref());
    let signature_str = format!("ed25519-{}", signature.to_base64());
    lockfile.metadata.lockfile_hash = lockfile_hash_str.clone();

    let signature_file =
        SignatureFile::new(lockfile_hash_str, signature_str, key_pair.key_id.clone());
    let signature_path = lockfile_path.with_extension("lock.sig");
    let temp_signature_path = signature_path.with_extension("lock.sig.tmp");
    std::fs::write(&temp_signature_path, signature_file.to_string())?;
    std::fs::rename(&temp_signature_path, &signature_path)?;

    Ok(())
}

/// Sign with the default key from the user keyring.
/// Ký bằng key mặc định trong keyring của user.
pub fn sign_lockfile_with_default_key(
    lockfile: &mut Lockfile,
    lockfile_path: &Path,
) -> LockfileResult<()> {
    let keyring = Keyring::init_if_not_exists().map_err(LockfileError::CryptoError)?;
    let key_pair = keyring
        .default_key()
        .ok_or_else(|| LockfileError::from(LegacySigningError::MissingDefaultKey))?;
    sign_and_write_lockfile(lockfile, lockfile_path, key_pair)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
