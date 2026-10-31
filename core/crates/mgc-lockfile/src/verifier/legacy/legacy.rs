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
            warn_on_digest_mismatch(lockfile_path, &lockfile);
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

/// Self-hash check for unsigned v3 locks: a recorded `lockfile_hash` must
/// recompute from the document itself. Locks written before the digest was
/// populated carry `""` (legacy, accepted silently — they stay `Unsigned`).
/// A non-empty digest that does not match is a real integrity failure, so it
/// is surfaced loudly instead of passing as a clean unsigned lock.
/// (Tự-kiểm hash cho lock v3 không ký: `lockfile_hash` đã ghi phải tính lại
/// được từ chính tài liệu. Lock cũ ghi `""` vẫn hợp lệ (giữ `Unsigned`).
/// Digest khác rỗng mà không khớp là lỗi toàn vẹn thật → cảnh báo rõ.)
fn warn_on_digest_mismatch(lockfile_path: &Path, lockfile: &crate::Lockfile) {
    if lockfile.metadata.lockfile_hash.is_empty() {
        return;
    }
    let Ok(bytes) = crate::parser::read_lockfile_bytes(lockfile_path) else {
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        return;
    };
    match crate::writer::recompute_lockfile_hash(text) {
        Ok(recomputed) if recomputed != lockfile.metadata.lockfile_hash => eprintln!(
            "WARNING: mgc.lock lockfile_hash does not match its content \
             (recorded {}, recomputed {recomputed}); the document was modified after it was written",
            lockfile.metadata.lockfile_hash
        ),
        Ok(_) => {}
        Err(error) => eprintln!("WARNING: mgc.lock self-hash could not be verified: {error}"),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
