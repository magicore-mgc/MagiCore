//! Read and decompress one local-file payload — đọc và giải nén payload của một file.

use mgc_types::{MgError, MgResult};

/// Read and decompress one entry's payload via its local header. Sizes come
/// from the central directory (local ones are zero with data descriptors).
/// Đọc và giải nén payload của một entry qua local header. Kích thước lấy từ
/// central directory (local header = 0 khi có data descriptor).
pub(in crate::protocols::zip_reader) fn read_local_entry(
    bytes: &[u8],
    offset: usize,
    method: u16,
    comp_size: u32,
    uncomp_size: u32,
    _flags: u16,
) -> MgResult<Vec<u8>> {
    let Some(header_end) = offset.checked_add(30) else {
        return Err(MgError::Other("zip local header out of bounds".to_string()));
    };
    if header_end > bytes.len() {
        return Err(MgError::Other("zip local header out of bounds".to_string()));
    }
    if &bytes[offset..offset + 4] != b"PK\x03\x04" {
        return Err(MgError::Other(
            "zip local header signature mismatch".to_string(),
        ));
    }
    let local_name_len = u16::from_le_bytes([bytes[offset + 26], bytes[offset + 27]]) as usize;
    let local_extra_len = u16::from_le_bytes([bytes[offset + 28], bytes[offset + 29]]) as usize;
    let Some(data_start) = header_end
        .checked_add(local_name_len)
        .and_then(|name_end| name_end.checked_add(local_extra_len))
    else {
        return Err(MgError::Other("zip local header out of bounds".to_string()));
    };
    let data_end = data_start
        .checked_add(comp_size as usize)
        .ok_or_else(|| MgError::Other("zip entry size overflow".to_string()))?;
    if data_end > bytes.len() {
        return Err(MgError::Other(
            "zip entry data exceeds archive bounds".to_string(),
        ));
    }
    let compressed = &bytes[data_start..data_end];
    match method {
        // Method 0 stores bytes unchanged — method 0 giữ nguyên byte đầu vào.
        0 => {
            ensure_uncompressed_size(uncomp_size, comp_size as usize)?;
            Ok(compressed.to_vec())
        }
        // Method 8 uses raw DEFLATE — method 8 dùng DEFLATE thô.
        8 => {
            let decoder = flate2::read::DeflateDecoder::new(compressed);
            let initial_capacity = (comp_size as usize)
                .min(uncomp_size as usize)
                .min(64 * 1024);
            let mut out = Vec::with_capacity(initial_capacity);
            let output_limit = u64::from(uncomp_size).saturating_add(1);
            let mut bounded = std::io::Read::take(decoder, output_limit);
            std::io::Read::read_to_end(&mut bounded, &mut out)
                .map_err(|e| MgError::Other(format!("zip deflate inflate failed: {e}")))?;
            ensure_uncompressed_size(uncomp_size, out.len())?;
            Ok(out)
        }
        other => Err(MgError::Other(format!(
            "unsupported zip compression method {other} (honest limit: store/deflate only)"
        ))),
    }
}

/// Require decoded payload length to match central-directory metadata.
/// Yêu cầu độ dài payload sau giải mã khớp metadata central-directory.
fn ensure_uncompressed_size(expected: u32, actual: usize) -> MgResult<()> {
    if actual != expected as usize {
        return Err(MgError::Integrity(format!(
            "zip entry uncompressed size mismatch: expected {expected}, got {actual}"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/local_entry.rs"]
mod tests;
