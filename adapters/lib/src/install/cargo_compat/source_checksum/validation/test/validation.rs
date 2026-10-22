//! Validate SHA-256 input shape at the nearest owner.
//! Kiểm tra dạng input SHA-256 tại owner gần nhất.

use super::super::implementation::validate_sha256;

#[test]
fn accepts_lowercase_and_uppercase_sha256() {
    assert!(validate_sha256(&"ab".repeat(32)).is_ok());
    assert!(validate_sha256(&"AB".repeat(32)).is_ok());
}

#[test]
fn rejects_short_and_non_hex_digests() {
    assert!(validate_sha256("abc").is_err());
    assert!(validate_sha256(&format!("{}z", "a".repeat(63))).is_err());
}
