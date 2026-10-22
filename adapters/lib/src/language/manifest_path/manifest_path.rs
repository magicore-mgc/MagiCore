//! Safe regular-file checks for library project manifests.
//! Kiểm tra file thường an toàn cho manifest project library.

use std::path::Path;

/// Do not follow symlinks when deciding project ownership.
/// Không theo symlink khi xác định file thuộc project.
pub(crate) fn is_regular_manifest(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
