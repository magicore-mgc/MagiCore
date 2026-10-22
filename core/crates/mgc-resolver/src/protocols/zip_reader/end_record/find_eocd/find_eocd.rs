//! Locate the End-Of-Central-Directory record — tìm record EOCD.

use mgc_types::{MgError, MgResult};

/// Locate the End-Of-Central-Directory record (scan the last 64 KiB + 22).
/// Tìm bản ghi End-Of-Central-Directory (quét 64 KiB cuối + 22).
pub(in crate::protocols::zip_reader::end_record) fn find_eocd(bytes: &[u8]) -> MgResult<usize> {
    if bytes.len() < 22 {
        return Err(MgError::Other("zip too small for an EOCD".to_string()));
    }
    let min_pos = bytes.len().saturating_sub(22 + 65_535);
    let mut i = bytes.len() - 22;
    loop {
        if &bytes[i..i + 4] == b"PK\x05\x06" {
            return Ok(i);
        }
        if i == min_pos {
            return Err(MgError::Other(
                "zip end-of-central-directory signature not found".to_string(),
            ));
        }
        i -= 1;
    }
}
