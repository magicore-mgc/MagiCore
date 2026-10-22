//! Verifies v4 inline signatures and rejects legacy sidecars.
//! Xác minh chữ ký inline v4 và từ chối sidecar legacy.

use std::path::Path;

use crate::{LockfileError, LockfileResult, canonical::LockfileV4};

use super::super::status::VerificationStatus;

pub(in crate::verifier) fn verify_v4_lockfile(
    lockfile_path: &Path,
    lockfile: &LockfileV4,
    trust_keys: Option<&[String]>,
) -> LockfileResult<VerificationStatus> {
    let signature_path = lockfile_path.with_extension("lock.sig");
    match std::fs::symlink_metadata(&signature_path) {
        Ok(_) => {
            return Ok(VerificationStatus::InvalidSignature(
                "v4 lockfile uses an inline signature; a legacy signature sidecar is not allowed"
                    .to_string(),
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    let report = match crate::policy::verify_v4_math(lockfile) {
        Ok(report) => report,
        Err(LockfileError::CryptoError(error)) => {
            return Ok(VerificationStatus::InvalidSignature(error.to_string()));
        }
        Err(LockfileError::VerificationFailed(message)) => {
            return Ok(VerificationStatus::InvalidSignature(message));
        }
        Err(error) => return Err(error),
    };
    if !report.digest_ok {
        return Ok(VerificationStatus::Tampered(
            "v4 payload digest does not match metadata.lockfile_hash".to_string(),
        ));
    }
    if !report.signed {
        return Ok(VerificationStatus::Unsigned);
    }
    if !report.signature_ok {
        return Ok(VerificationStatus::InvalidSignature(
            "v4 Ed25519 signature does not verify".to_string(),
        ));
    }
    let signer_id = report.key_id.unwrap_or_default();
    if trust_keys.is_some_and(|keys| !keys.iter().any(|key| key == &signer_id)) {
        return Ok(VerificationStatus::UntrustedKey(signer_id));
    }
    Ok(VerificationStatus::Valid)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
