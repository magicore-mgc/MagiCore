//! Derives signature sidecar paths and probes bounded file presence.
//! Tạo path sidecar chữ ký và kiểm tra file có giới hạn.

use std::path::Path;

use crate::{LockfileError, LockfileResult};

/// Check sidecar presence without treating invalid paths as unsigned.
/// Kiểm tra sidecar, không coi path sai là lock chưa ký.
pub fn signature_file_presence(lockfile_path: &Path) -> LockfileResult<bool> {
    let signature_path = signature_path_for(lockfile_path);
    match super::super::bounded_reader::read_signature_file_bytes(&signature_path) {
        Ok(_) => Ok(true),
        Err(LockfileError::IoError(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(false)
        }
        Err(error) => Err(error),
    }
}

/// Get the `.lock.sig` sidecar path for a lockfile path.
/// Lấy path sidecar `.lock.sig` tương ứng với lockfile.
pub fn signature_path_for(lockfile_path: &Path) -> std::path::PathBuf {
    lockfile_path.with_extension("lock.sig")
}

/// Compatibility wrapper; use `signature_file_presence` to retain errors.
/// Hàm tương thích; dùng `signature_file_presence` để giữ thông tin lỗi.
#[deprecated(note = "use signature_file_presence to distinguish missing from invalid paths")]
pub fn is_lockfile_signed(lockfile_path: &Path) -> bool {
    signature_file_presence(lockfile_path).unwrap_or(false)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
