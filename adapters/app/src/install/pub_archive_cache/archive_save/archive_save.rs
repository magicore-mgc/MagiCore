//! Verify and atomically publish a Pub archive into the project cache.
//! Xác minh và ghi atomic archive Pub vào cache dự án.
use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::archive_path::archive_path;
use super::super::archive_policy::{ARCHIVE_MAX_BYTES_ENV, archive_limit};
use super::super::archive_verify::verify;

pub(in crate::install::pub_archive_cache) fn save_verified(
    root: &Path,
    expected: &str,
    bytes: &[u8],
) -> MgResult<()> {
    let path = archive_path(root, expected)?;
    verify(expected, bytes)?;
    if bytes.len() as u64 > archive_limit()? {
        return Err(MgError::Integrity(format!(
            "Pub archive exceeds {ARCHIVE_MAX_BYTES_ENV} limit"
        )));
    }
    let directory = path
        .parent()
        .ok_or_else(|| MgError::Other("Pub archive cache has no parent".into()))?;
    std::fs::create_dir_all(directory)
        .map_err(|error| MgError::Other(format!("create Pub archive cache: {error}")))?;
    crate::install::atomic_write_project_file(&path, bytes, "Pub archive")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
