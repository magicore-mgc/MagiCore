//! Rejects symlinks and special files as project markers.
//! Từ chối symlink và file đặc biệt làm marker project.

use std::path::Path;

/// Returns true only for a non-symlink regular file.
/// Chỉ trả true với file thường không phải symlink.
pub(crate) fn is_regular_manifest(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}
