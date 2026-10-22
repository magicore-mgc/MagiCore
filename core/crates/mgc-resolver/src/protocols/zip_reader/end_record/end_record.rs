//! EOCD lookup and field decoding — tìm EOCD và giải mã các trường.

use mgc_types::MgResult;

/// Return entry count, central-directory offset and size from EOCD.
/// Trả số entry, offset và kích thước central directory từ EOCD.
pub(in crate::protocols::zip_reader) fn read_end_record(
    bytes: &[u8],
) -> MgResult<(usize, u32, u32)> {
    let eocd = super::find_eocd::find_eocd(bytes)?;
    super::parse_eocd::parse_eocd(bytes, eocd)
}

#[cfg(test)]
#[path = "test/end_record.rs"]
mod tests;
