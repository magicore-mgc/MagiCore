//! Adler-32 verification wrapper — Wrapper xác minh Adler-32.
//! Preserve the historical result type and comparison behavior — Giữ kiểu result và hành vi so sánh lịch sử.

use anyhow::Result;

use crate::checksum::checksum_adler32;

/// Compare the calculated checksum with the expected value.
/// So sánh checksum vừa tính với giá trị kỳ vọng.
pub fn verify_adler32(data: &[u8], expected: u32) -> Result<bool> {
    Ok(checksum_adler32(data) == expected)
}
