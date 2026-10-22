// Lowercase BLAKE3 hexadecimal validation behavior.
// Hành vi validation hexadecimal BLAKE3 chữ thường.
use super::error::{BLAKE3_HEX_LEN, InvalidHashError};

/// Validate a string as a lowercase 64-char BLAKE3 hex digest.
/// Kiểm tra chuỗi có phải hex thường 64 ký tự BLAKE3 không.
pub fn validate_blake3_hex(hash_hex: &str) -> Result<(), InvalidHashError> {
    if hash_hex.len() == BLAKE3_HEX_LEN
        && hash_hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        Ok(())
    } else {
        Err(InvalidHashError {
            input: hash_hex.to_string(),
        })
    }
}
