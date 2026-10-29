//! Stream, validate and extract one gzip-compressed tarball.
//! Đọc stream, xác minh và giải nén một tarball gzip.

use super::super::archive_limits::{MAX_ARCHIVE_ENTRIES, MAX_ENTRY_BYTES, MAX_TOTAL_FILE_BYTES};
use super::super::{ExtractionError, ExtractionResult};
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use tar::Archive;

/// Extracts a gzip-compressed tarball from an arbitrary reader.
/// Giải nén tarball gzip từ reader bất kỳ.
pub fn extract_tarball_from_reader<R: Read>(reader: R, dest: &Path) -> anyhow::Result<()> {
    extract_reader(reader, dest).map_err(anyhow::Error::new)
}

fn extract_reader<R: Read>(reader: R, dest: &Path) -> ExtractionResult<()> {
    let decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(decoder);
    let dest_root = super::super::extract_target::ensure_extract_root(dest)?;
    let mut entry_count = 0usize;
    let mut total_unpacked_bytes = 0u64;

    for entry in archive.entries()? {
        let mut entry = entry?;
        entry_count = entry_count
            .checked_add(1)
            .ok_or(ExtractionError::EntryCountOverflow)?;
        if entry_count > MAX_ARCHIVE_ENTRIES {
            return Err(ExtractionError::EntryLimit);
        }
        let entry_size = entry.size();
        if entry_size > MAX_ENTRY_BYTES {
            return Err(ExtractionError::EntryTooLarge);
        }
        total_unpacked_bytes = total_unpacked_bytes
            .checked_add(entry_size)
            .ok_or(ExtractionError::TotalSizeOverflow)?;
        if total_unpacked_bytes > MAX_TOTAL_FILE_BYTES {
            return Err(ExtractionError::TotalSizeLimit);
        }
        let relative = super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let entry_type = entry.header().entry_type();
        // SEC-3: refuse interpreter install hooks before the first write.
        // (SEC-3: từ chối hook cài đặt interpreter trước lần ghi đầu.)
        super::super::install_hook_guard::InstallHookGuard::deny().check(
            &relative,
            entry.header().mode().ok(),
        )?;
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ExtractionError::LinksNotAllowed(dest_root.join(&relative)));
        }
        let target = dest_root.join(&relative);
        if !target.starts_with(&dest_root) {
            return Err(ExtractionError::PathEscapes(target));
        }
        if entry_type.is_dir() {
            super::super::extract_target::ensure_extract_directory(&dest_root, &relative)?;
            continue;
        }
        // Npm pax entries contain bookkeeping metadata, not package files.
        // Entry pax chỉ là metadata archive, không phải file package.
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        if !entry_type.is_file() {
            return Err(ExtractionError::UnsupportedEntry(target));
        }
        let target =
            super::super::extract_target::ensure_extract_file_target(&dest_root, &relative)?;
        // Tar removes an existing final entry before creating a new file with
        // create_new, so an existing hard link is unlinked rather than written
        // through. The target and its parents were checked above.
        // Tar xóa entry đích cũ rồi tạo file mới bằng create_new, nên hard link
        // cũ chỉ bị bỏ khỏi path này chứ không bị ghi xuyên qua.
        entry.unpack(&target)?;
    }
    Ok(())
}
