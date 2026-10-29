//! Stable archive API boundary and shared extraction limits.
//! Boundary API archive ổn định và giới hạn giải nén dùng chung.

use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub(in crate::protocols::archive) use super::super::archive_limits::MAX_TAR_ENTRIES;
pub(in crate::protocols::archive) use super::super::archive_limits::MAX_TAR_ENTRY_UNCOMPRESSED;
pub(in crate::protocols::archive) use super::super::archive_limits::MAX_TAR_TOTAL_UNCOMPRESSED;

/// Extract a gzip-compressed tarball held in memory into the destination.
/// SEC-3: interpreter install-hook entries (`.pth`, `sitecustomize.py`,
/// `install.ps1`) and setuid/setgid members are refused unless the caller
/// named this package in `MAGICORE_ALLOW_INSTALL_HOOKS`.
/// (Giải nén tarball gzip trong bộ nhớ vào thư mục đích. SEC-3: entry hook cài
/// đặt của interpreter và thành viên setuid/setgid bị từ chối trừ khi caller
/// nêu package trong `MAGICORE_ALLOW_INSTALL_HOOKS`.)
pub fn extract_tar_gz(bytes: &[u8], dest: &Path) -> MgResult<()> {
    super::super::tar_extraction::extract_tar_gz(
        bytes,
        dest,
        &super::super::InstallHookPolicy::deny(),
    )
    .map_err(MgError::from)
}

/// Extract a tar.gz while allowing install hooks for one explicitly named
/// package (operator escape hatch; the caller must warn loudly).
/// Giải nén tar.gz và cho phép hook cài đặt cho một package được nêu tường minh
/// (escape hatch của operator; caller phải cảnh báo rõ).
#[allow(dead_code)]
pub fn extract_tar_gz_allow_hooks(bytes: &[u8], dest: &Path, package: &str) -> MgResult<()> {
    super::super::tar_extraction::extract_tar_gz(
        bytes,
        dest,
        &super::super::InstallHookPolicy::for_package(package),
    )
    .map_err(MgError::from)
}

/// Validate and extract a Cargo archive under its exact package root.
/// Cargo archives are published source, and the registry rejects `.pth` files,
/// so the install-hook gate applies here too.
/// (Xác minh và giải nén archive Cargo dưới đúng thư mục gốc package. Archive
/// Cargo là source đã publish và registry từ chối file `.pth`, nên cổng hook
/// vẫn áp dụng.)
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
