//! Projects full verification status into a tampered boolean.
//! Chiếu status xác minh đầy đủ thành giá trị tampered boolean.

use std::path::Path;

use crate::LockfileResult;

/// Quick tamper check; this still performs full lockfile verification.
/// Kiểm tra tamper nhanh; hàm vẫn chạy xác minh lockfile đầy đủ.
pub fn is_lockfile_tampered(lockfile_path: &Path) -> LockfileResult<bool> {
    let status = super::super::verifier::verify_lockfile(lockfile_path)?;
    Ok(matches!(
        status,
        super::super::status::VerificationStatus::Tampered(_)
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
