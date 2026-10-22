//! Parse central entry and validate payload integrity — đọc entry trung tâm và kiểm toàn vẹn.

use super::super::crc32::crc32;
use super::super::limits::MAX_ENTRY_UNCOMPRESSED;
use super::super::local_entry::read_local_entry;
use super::super::zip_entry::ZipEntry;
use mgc_types::{MgError, MgResult};

/// Parse one central-directory entry and extract its file content via the
/// local header. Returns `Ok(None)` for directory entries.
/// Parse một entry central-directory và lấy nội dung file qua local header.
/// Trả `Ok(None)` cho entry thư mục.
pub(in crate::protocols::zip_reader) fn read_central_entry(
    bytes: &[u8],
    pos: usize,
    directory_end: usize,
    remaining_uncompressed: u64,
) -> MgResult<(Option<ZipEntry>, usize)> {
    let Some(name_start) = pos.checked_add(46) else {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    };
    if name_start > bytes.len() || name_start > directory_end {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    }
    if &bytes[pos..pos + 4] != b"PK\x01\x02" {
        return Err(MgError::Other(
            "zip central directory entry signature mismatch".to_string(),
        ));
    }
    let flags = u16::from_le_bytes([bytes[pos + 8], bytes[pos + 9]]);
    let method = u16::from_le_bytes([bytes[pos + 10], bytes[pos + 11]]);
    let expected_crc = u32::from_le_bytes([
        bytes[pos + 16],
        bytes[pos + 17],
        bytes[pos + 18],
        bytes[pos + 19],
    ]);
    let comp_size = u32::from_le_bytes([
        bytes[pos + 20],
        bytes[pos + 21],
        bytes[pos + 22],
        bytes[pos + 23],
    ]);
    let uncomp_size = u32::from_le_bytes([
        bytes[pos + 24],
        bytes[pos + 25],
        bytes[pos + 26],
        bytes[pos + 27],
    ]);
    let name_len = u16::from_le_bytes([bytes[pos + 28], bytes[pos + 29]]) as usize;
    let extra_len = u16::from_le_bytes([bytes[pos + 30], bytes[pos + 31]]) as usize;
    let comment_len = u16::from_le_bytes([bytes[pos + 32], bytes[pos + 33]]) as usize;
    let local_offset = u32::from_le_bytes([
        bytes[pos + 42],
        bytes[pos + 43],
        bytes[pos + 44],
        bytes[pos + 45],
    ]) as usize;
    let Some(name_end) = name_start.checked_add(name_len) else {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    };
    let Some(extra_end) = name_end.checked_add(extra_len) else {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    };
    let Some(next) = extra_end.checked_add(comment_len) else {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    };
    if next > bytes.len() || next > directory_end {
        return Err(MgError::Other(
            "zip central directory truncated".to_string(),
        ));
    }

    if uncomp_size == u32::MAX || comp_size == u32::MAX || local_offset == u32::MAX as usize {
        return Err(MgError::Other(
            "zip64 archives are not supported (honest limit)".to_string(),
        ));
    }
    let name = std::str::from_utf8(&bytes[name_start..name_end])
        .map_err(|_| MgError::Other("zip entry name is not UTF-8".to_string()))?
        .to_string();
    // Directory entries end with '/' — nothing to extract.
    // (Entry thư mục kết thúc '/' — không cần giải nén.)
    if name.ends_with('/') {
        return Ok((None, next));
    }
    if uncomp_size as u64 > MAX_ENTRY_UNCOMPRESSED {
        return Err(MgError::Other(format!(
            "zip entry '{name}' exceeds the per-entry size limit"
        )));
    }
    if uncomp_size as u64 > remaining_uncompressed {
        return Err(MgError::Other(
            "zip total uncompressed size exceeds the safety limit".to_string(),
        ));
    }

    let data = read_local_entry(bytes, local_offset, method, comp_size, uncomp_size, flags)?;
    let actual_crc = crc32(&data);
    if actual_crc != expected_crc {
        return Err(MgError::Integrity(format!(
            "zip entry '{name}' CRC-32 mismatch: expected {expected_crc:#010x}, got {actual_crc:#010x}"
        )));
    }
    Ok((Some(ZipEntry { name, data }), next))
}

#[cfg(test)]
#[path = "test/central_entry.rs"]
mod tests;
