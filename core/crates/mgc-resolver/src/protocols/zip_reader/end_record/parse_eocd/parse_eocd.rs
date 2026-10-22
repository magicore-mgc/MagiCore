//! Decode EOCD fields — giải mã các trường của EOCD.

use mgc_types::MgResult;

/// EOCD → (entry count, central-directory offset, central-directory size).
pub(in crate::protocols::zip_reader::end_record) fn parse_eocd(
    bytes: &[u8],
    eocd: usize,
) -> MgResult<(usize, u32, u32)> {
    let entry_count = u16::from_le_bytes([bytes[eocd + 10], bytes[eocd + 11]]) as usize;
    let cd_size = u32::from_le_bytes([
        bytes[eocd + 12],
        bytes[eocd + 13],
        bytes[eocd + 14],
        bytes[eocd + 15],
    ]);
    let cd_offset = u32::from_le_bytes([
        bytes[eocd + 16],
        bytes[eocd + 17],
        bytes[eocd + 18],
        bytes[eocd + 19],
    ]);
    Ok((entry_count, cd_offset, cd_size))
}
