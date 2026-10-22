//! Coordinates version dispatch for lockfile verification.
//! Điều phối dispatch version cho xác minh lockfile.

use std::path::Path;

use crate::{LockDocument, LockfileResult};

use super::super::status::VerificationStatus;

/// Verify a version-dispatched lockfile's integrity and signature.
/// Xác minh toàn vẹn và chữ ký lockfile theo schema.
pub fn verify_lockfile(lockfile_path: &Path) -> LockfileResult<VerificationStatus> {
    verify_lockfile_inner(lockfile_path, None)
}

/// Verify a lockfile and require its signer to appear in project trust roots.
/// Xác minh lockfile và yêu cầu signer thuộc trust roots của project.
pub fn verify_lockfile_with_trust(
    lockfile_path: &Path,
    trust_keys: &[String],
) -> LockfileResult<VerificationStatus> {
    verify_lockfile_inner(lockfile_path, Some(trust_keys))
}

fn verify_lockfile_inner(
    lockfile_path: &Path,
    trust_keys: Option<&[String]>,
) -> LockfileResult<VerificationStatus> {
    match crate::parser::load_lock_document(lockfile_path)? {
        LockDocument::V4(lock) => {
            super::super::v4::verify_v4_lockfile(lockfile_path, &lock, trust_keys)
        }
        LockDocument::Legacy(_) => {
            super::super::legacy::verify_legacy_lockfile(lockfile_path, trust_keys)
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
