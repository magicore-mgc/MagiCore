//! ZIP directory read coordinator — điều phối đọc directory ZIP.

use super::central_entry::read_central_entry;
use super::end_record::read_end_record;
use super::limits::{MAX_ENTRIES, MAX_TOTAL_UNCOMPRESSED};
use super::zip_entry::ZipEntry;
use mgc_types::{MgError, MgResult};

/// Read every regular-file entry of a ZIP archive held in memory.
/// Directories are skipped; zip64 records are rejected honestly.
/// Đọc mọi entry file thường của ZIP trong bộ nhớ. Bỏ qua thư mục;
/// zip64 bị từ chối trung thực.
pub fn read_zip_entries(bytes: &[u8]) -> MgResult<Vec<ZipEntry>> {
    let (entry_count, cd_offset, cd_size) = read_end_record(bytes)?;
    if entry_count > MAX_ENTRIES {
        return Err(MgError::Other(format!(
            "zip has {entry_count} entries, over the {MAX_ENTRIES} limit"
        )));
    }
    let cd_start = cd_offset as usize;
    let cd_end = cd_start
        .checked_add(cd_size as usize)
        .ok_or_else(|| MgError::Other("zip central directory size overflow".to_string()))?;
    if cd_end > bytes.len() {
        return Err(MgError::Other(
            "zip central directory exceeds archive bounds".to_string(),
        ));
    }

    let mut entries = Vec::with_capacity(entry_count.min(1024));
    let mut total_uncompressed: u64 = 0;
    let mut pos = cd_start;
    for _ in 0..entry_count {
        let remaining_uncompressed = MAX_TOTAL_UNCOMPRESSED.saturating_sub(total_uncompressed);
        let (entry, next) = read_central_entry(bytes, pos, cd_end, remaining_uncompressed)?;
        pos = next;
        let Some(entry) = entry else { continue };
        total_uncompressed += entry.data.len() as u64;
        if total_uncompressed > MAX_TOTAL_UNCOMPRESSED {
            return Err(MgError::Other(
                "zip total uncompressed size exceeds the safety limit".to_string(),
            ));
        }
        entries.push(entry);
    }
    Ok(entries)
}

#[cfg(test)]
#[path = "test/zip_reader.rs"]
mod tests;
