//! Validate and derive a Pub archive cache path.
//! Kiểm tra và tạo đường dẫn cache archive Pub.
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

use super::super::archive_policy::ARCHIVE_DIRECTORY;

pub(in crate::install::pub_archive_cache) fn archive_path(
    root: &Path,
    expected: &str,
) -> MgResult<PathBuf> {
    if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(MgError::Integrity(
            "Pub archive cache requires a valid SHA-256 digest".into(),
        ));
    }
    let directory = root.join(ARCHIVE_DIRECTORY);
    match std::fs::symlink_metadata(&directory) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => {
            return Err(MgError::Integrity(
                "refusing linked or non-directory Pub archive cache".into(),
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(MgError::Other(format!(
                "inspect Pub archive cache: {error}"
            )));
        }
    }
    Ok(directory.join(format!("{}.tar.gz", expected.to_ascii_lowercase())))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
