//! Verifies legacy lockfiles with signature sidecars.
//! Xác minh lockfile legacy dùng sidecar chữ ký.

use std::path::Path;

use crate::{LockfileError, LockfileResult};

use super::super::status::VerificationStatus;

pub(in crate::verifier) fn verify_legacy_lockfile(
    lockfile_path: &Path,
    trust_keys: Option<&[String]>,
) -> LockfileResult<VerificationStatus> {
    let signature_path = lockfile_path.with_extension("lock.sig");
    match std::fs::symlink_metadata(&signature_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let lockfile = crate::parser::load_lockfile(lockfile_path)?;
            if lockfile.metadata.signer.is_some() {
                return Ok(VerificationStatus::InvalidSignature(
                    "lockfile declares a signer but its signature sidecar is missing".to_string(),
                ));
            }
            return Ok(VerificationStatus::Unsigned);
        }
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }

    match crate::parser::load_and_verify_lockfile(lockfile_path, &signature_path) {
        Ok(lockfile) => {
            let signer_id = lockfile
                .metadata
                .signer
                .as_ref()
                .map(|signer| signer.key_id.as_str())
                .unwrap_or_default();
            if trust_keys.is_some_and(|keys| !keys.iter().any(|key| key == signer_id)) {
                Ok(VerificationStatus::UntrustedKey(signer_id.to_string()))
            } else {
                Ok(VerificationStatus::Valid)
            }
        }
        Err(LockfileError::TamperedLockfile(message)) => Ok(VerificationStatus::Tampered(message)),
        Err(LockfileError::VerificationFailed(message)) => {
            Ok(VerificationStatus::InvalidSignature(message))
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
