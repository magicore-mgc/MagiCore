//! Stable archive API boundary and shared extraction limits.
//! Boundary API archive ổn định và giới hạn giải nén dùng chung.

use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub(in crate::protocols::archive) const MAX_TAR_ENTRY_UNCOMPRESSED: u64 = 256 * 1024 * 1024;
pub(in crate::protocols::archive) const MAX_TAR_TOTAL_UNCOMPRESSED: u64 = 1024 * 1024 * 1024;

/// Extract a gzip-compressed tarball held in memory into the destination.
/// Giải nén tarball gzip trong bộ nhớ vào thư mục đích.
pub fn extract_tar_gz(bytes: &[u8], dest: &Path) -> MgResult<()> {
    super::super::tar_extraction::extract_tar_gz(bytes, dest).map_err(MgError::from)
}

/// Validate and extract a Cargo archive under its exact package root.
/// Xác minh và giải nén archive Cargo dưới đúng thư mục gốc package.
pub fn extract_crate_tar_gz(bytes: &[u8], dest: &Path, expected_root: &str) -> MgResult<()> {
    super::super::crate_extraction::extract_crate_tar_gz(bytes, dest, expected_root)
        .map_err(MgError::from)
}

/// Hash source files from a validated Cargo archive without extracting them.
/// Hash source file trực tiếp từ archive Cargo đã xác minh, không giải nén.
pub fn cargo_source_file_hashes(
    bytes: &[u8],
    expected_root: &str,
) -> MgResult<std::collections::BTreeMap<String, String>> {
    super::super::cargo_source_hashes::cargo_source_file_hashes(bytes, expected_root)
        .map_err(MgError::from)
}

pub(crate) fn ensure_extract_root(dest: &Path) -> MgResult<PathBuf> {
    super::super::extract_root::ensure_extract_root(dest).map_err(MgError::from)
}

pub(crate) fn ensure_extract_file_target(root: &Path, relative: &Path) -> MgResult<PathBuf> {
    super::super::extract_target::ensure_extract_file_target(root, relative).map_err(MgError::from)
}

pub(crate) fn write_extracted_file(path: &Path, contents: &[u8]) -> MgResult<()> {
    super::super::extracted_file_write::write_extracted_file(path, contents).map_err(MgError::from)
}
