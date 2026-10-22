//! Acquire a real non-linked extraction root.
//! Lấy thư mục gốc giải nén thật và không phải liên kết.

use super::super::{ArchiveError, ArchiveResult};
use std::path::{Path, PathBuf};

pub(in crate::protocols::archive) fn ensure_extract_root(dest: &Path) -> ArchiveResult<PathBuf> {
    match std::fs::symlink_metadata(dest) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(ArchiveError::Integrity(format!(
                "refusing linked or non-directory archive extraction root: {}",
                dest.display()
            )));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir_all(dest)?;
        }
        Err(error) => return Err(error.into()),
    }
    let metadata = std::fs::symlink_metadata(dest)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ArchiveError::Integrity(format!(
            "archive extraction root changed to a link or non-directory: {}",
            dest.display()
        )));
    }
    Ok(dest.canonicalize()?)
}
