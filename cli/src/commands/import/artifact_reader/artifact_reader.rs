//! Safely snapshots optional existing import artifacts.
//! Chụp an toàn artifact import hiện có nếu tồn tại.

use super::super::ImportError;
use std::path::Path;

/// Read an optional existing lockfile through the lockfile safety gate.
/// Đọc lockfile cũ tùy chọn qua cổng an toàn lockfile.
pub(in crate::commands::import) fn read_optional_lock(
    path: &Path,
) -> Result<Option<Vec<u8>>, ImportError> {
    match mgc_lockfile::read_lockfile_bytes(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            Ok(None)
        }
        Err(source) => Err(ImportError::LockfileRead {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// Read an optional existing signature through the lockfile safety gate.
/// Đọc signature cũ tùy chọn qua cổng an toàn lockfile.
pub(in crate::commands::import) fn read_optional_signature(
    path: &Path,
) -> Result<Option<Vec<u8>>, ImportError> {
    match mgc_lockfile::read_signature_file_bytes(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            Ok(None)
        }
        Err(source) => Err(ImportError::SignatureRead {
            path: path.to_path_buf(),
            source,
        }),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
