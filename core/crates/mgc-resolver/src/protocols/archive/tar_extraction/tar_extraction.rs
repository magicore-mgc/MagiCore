//! General safe tar.gz extraction.
//! Giải nén tar.gz tổng quát theo quy tắc an toàn.

use super::super::archive::{MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED};
use super::super::{ArchiveError, ArchiveResult};
use std::io::Read as _;
use std::path::Path;

/// Extract a gzip-compressed tarball (held in memory) into `dest`.
/// Giải nén tarball gzip (nằm trong bộ nhớ) vào `dest`.
pub(in crate::protocols::archive) fn extract_tar_gz(
    bytes: &[u8],
    dest: &Path,
) -> ArchiveResult<()> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);

    let dest_root = super::super::extract_root::ensure_extract_root(dest)?;
    let mut total_uncompressed = 0_u64;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let rel_path = super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ArchiveError::Other(format!(
                "tar links are not allowed: {}",
                rel_path.display()
            )));
        }
        let target = if entry_type.is_dir() {
            super::super::extract_target::ensure_extract_directory(&dest_root, &rel_path)?
        } else {
            super::super::extract_target::ensure_extract_file_target(&dest_root, &rel_path)?
        };

        if !target.starts_with(&dest_root) {
            return Err(ArchiveError::Other(format!(
                "tar entry escapes destination: {}",
                target.display()
            )));
        }

        if entry_type.is_dir() {
            continue;
        }

        // pax metadata entries are archive bookkeeping, not package files.
        // (Entry metadata pax là bookkeeping archive, không phải file package.)
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }

        if !entry_type.is_file() {
            return Err(ArchiveError::Other(format!(
                "unsupported tar entry type for {}",
                target.display()
            )));
        }

        let size = entry.size();
        if size > MAX_TAR_ENTRY_UNCOMPRESSED {
            return Err(ArchiveError::Other(format!(
                "tar entry '{}' exceeds the per-entry size limit",
                rel_path.display()
            )));
        }
        total_uncompressed = total_uncompressed.checked_add(size).ok_or_else(|| {
            ArchiveError::Other("tar total uncompressed size overflow".to_string())
        })?;
        if total_uncompressed > MAX_TAR_TOTAL_UNCOMPRESSED {
            return Err(ArchiveError::Other(
                "tar total uncompressed size exceeds the safety limit".to_string(),
            ));
        }

        let mut contents = Vec::with_capacity(size as usize);
        entry.read_to_end(&mut contents)?;
        super::super::extracted_file_write::write_extracted_file(&target, &contents)?;
    }

    Ok(())
}
