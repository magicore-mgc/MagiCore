// Typed failure for a hash string outside the BLAKE3 hex contract.
// Lỗi typed khi chuỗi hash nằm ngoài hợp đồng hex BLAKE3.

/// BLAKE3 hex digest length (32 bytes -> 64 hex chars).
/// Độ dài digest hex BLAKE3 chuẩn (32 byte -> 64 ký tự hex).
pub const BLAKE3_HEX_LEN: usize = 64;

/// Error for a hash string that violates the BLAKE3 hex contract.
/// Lỗi khi chuỗi hash vi phạm hợp đồng hex BLAKE3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidHashError {
    pub input: String,
}

impl InvalidHashError {
    pub fn reason(&self) -> &'static str {
        if self.input.len() != BLAKE3_HEX_LEN {
            "expected 64 hex characters"
        } else {
            "expected lowercase hex characters only"
        }
    }
}

impl std::fmt::Display for InvalidHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid blake3 digest '{}': {}",
            self.input,
            self.reason()
        )
    }
}

impl std::error::Error for InvalidHashError {}
