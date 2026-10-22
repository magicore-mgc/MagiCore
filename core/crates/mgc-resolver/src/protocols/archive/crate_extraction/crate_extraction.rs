//! Cargo archive root validation and extraction.
//! Xác minh thư mục gốc Cargo và giải nén archive.

use super::super::archive::{MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED};
use super::super::{ArchiveError, ArchiveResult};
use std::path::Path;

/// Validate and extract a Cargo `.crate` archive whose entries all live under
/// the expected `{name}-{version}` root. Unlike the general tarball extractor,
/// the destination is the shared `registry/src` directory, so this prefix
/// check prevents one package archive from writing into another package's
/// source directory. No destination is touched unless every archive entry is
/// structurally valid and the package contains its root `Cargo.toml`.
/// Xác minh và giải nén archive Cargo dưới đúng thư mục gốc `{name}-{version}`.
/// Đích là `registry/src` dùng chung, vì vậy check prefix ngăn archive package
/// ghi vào source của package khác; không ghi gì trước khi toàn archive hợp lệ.
pub(in crate::protocols::archive) fn extract_crate_tar_gz(
    bytes: &[u8],
    dest: &Path,
    expected_root: &str,
) -> ArchiveResult<()> {
    use std::path::Component;

    let expected = Path::new(expected_root);
    if expected.components().count() != 1
        || !matches!(expected.components().next(), Some(Component::Normal(_)))
    {
        return Err(ArchiveError::Integrity(format!(
            "invalid Cargo crate archive root '{expected_root}'"
        )));
    }

    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    let mut saw_manifest = false;
    let mut total_uncompressed = 0_u64;

    for entry in archive.entries()? {
        let entry = entry?;
        let archive_path =
            super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let relative = archive_path.strip_prefix(expected).map_err(|_| {
            ArchiveError::Integrity(format!(
                "Cargo crate archive entry '{}' is outside expected root '{expected_root}'",
                archive_path.display()
            ))
        })?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive links are not allowed: {}",
                archive_path.display()
            )));
        }
        if relative.as_os_str().is_empty() {
            if entry_type.is_dir() {
                continue;
            }
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive root '{expected_root}' must be a directory"
            )));
        }
        if entry_type.is_dir() {
            continue;
        }
        // pax metadata entries are bookkeeping, not package files. Preserve
        // the same allowance as `extract_tar_gz` after validating their path.
        // Entry pax chỉ lưu metadata archive, không phải file package; giữ quy
        // tắc tương thích với `extract_tar_gz` sau khi xác minh đường dẫn.
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        if !entry_type.is_file() {
            return Err(ArchiveError::Integrity(format!(
                "unsupported Cargo crate archive entry type for {}",
                archive_path.display()
            )));
        }
        let size = entry.size();
        if size > MAX_TAR_ENTRY_UNCOMPRESSED {
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive entry '{}' exceeds the per-entry size limit",
                archive_path.display()
            )));
        }
        total_uncompressed = total_uncompressed.checked_add(size).ok_or_else(|| {
            ArchiveError::Integrity("Cargo crate archive size overflow".to_string())
        })?;
        if total_uncompressed > MAX_TAR_TOTAL_UNCOMPRESSED {
            return Err(ArchiveError::Integrity(
                "Cargo crate archive exceeds the total uncompressed size limit".to_string(),
            ));
        }
        if relative == Path::new("Cargo.toml") {
            saw_manifest = true;
        }
    }

    if !saw_manifest {
        return Err(ArchiveError::Integrity(format!(
            "Cargo crate archive '{expected_root}' has no root Cargo.toml"
        )));
    }

    super::super::tar_extraction::extract_tar_gz(bytes, dest)
}
