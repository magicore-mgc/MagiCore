//! Tests valid and malformed external BLAKE3 input.
//! Kiểm thử chuỗi BLAKE3 ngoài hợp lệ và sai định dạng.

use super::super::validate_external_hash;
use crate::cas::store::StoreError;

#[test]
fn accepts_a_full_hex_digest() {
    assert!(validate_external_hash(&"a".repeat(64)).is_ok());
}

#[test]
fn rejects_wrong_length_or_non_hex_input() {
    assert!(matches!(
        validate_external_hash("abc"),
        Err(StoreError::InvalidHash(_))
    ));
    assert!(matches!(
        validate_external_hash(&"g".repeat(64)),
        Err(StoreError::InvalidHash(_))
    ));
}
