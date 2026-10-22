//! Validate SafeTensors file header length and truncation boundaries.
//! Kiểm tra độ dài header và ranh giới file bị cắt của SafeTensors.

use crate::audit::Finding;
use mgc_types::MgResult;
use std::io::Read;
use std::path::Path;

/// Scan SafeTensors metadata boundaries without loading tensor payloads.
/// Quét ranh giới metadata SafeTensors mà không nạp payload tensor.
pub fn scan_safetensors(path: &Path) -> MgResult<Vec<Finding>> {
    const MAX_HEADER_BYTES: u64 = 100 * 1024 * 1024;
    let mut findings = Vec::new();
    let file_size = std::fs::metadata(path)?.len();

    // SafeTensors starts with an eight-byte header length followed by JSON.
    // SafeTensors bắt đầu bằng độ dài header 8 byte rồi tới JSON.
    if file_size < 8 {
        findings.push(
            Finding::critical("safetensors-format", "Invalid safetensors header")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    // Read the little-endian u64 header length exactly.
    // Đọc chính xác độ dài header u64 theo little-endian.
    let mut file = std::fs::File::open(path)?;
    let mut length_bytes = [0u8; 8];
    file.read_exact(&mut length_bytes)?;
    let header_len = u64::from_le_bytes(length_bytes);

    if header_len > MAX_HEADER_BYTES {
        findings.push(
            Finding::high("safetensors-header", "Suspiciously large metadata (>100MB)")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    let Some(header_end) = header_len.checked_add(8) else {
        findings.push(
            Finding::critical("safetensors-truncated", "Truncated or corrupted file")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    };
    if header_end > file_size {
        findings.push(
            Finding::critical("safetensors-truncated", "Truncated or corrupted file")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    Ok(findings)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
