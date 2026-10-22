// BLAKE3 hash input validation behavior tests.
// Test hành vi validation input hash BLAKE3.
use super::super::{BLAKE3_HEX_LEN, validate_blake3_hex};

#[test]
fn accepts_exact_lowercase_digest() {
    let hash = "a".repeat(BLAKE3_HEX_LEN);

    assert!(validate_blake3_hex(&hash).is_ok());
}

#[test]
fn rejects_uppercase_hex() {
    let hash = "A".repeat(BLAKE3_HEX_LEN);
    let error = validate_blake3_hex(&hash).expect_err("uppercase hex is outside the contract");

    assert_eq!(error.reason(), "expected lowercase hex characters only");
}

#[test]
fn rejects_wrong_length() {
    let hash = "a".repeat(BLAKE3_HEX_LEN - 1);
    let error = validate_blake3_hex(&hash).expect_err("short hashes are invalid");

    assert_eq!(error.reason(), "expected 64 hex characters");
}

#[test]
fn rejects_non_ascii_hex_characters() {
    let hash = "é".repeat(BLAKE3_HEX_LEN / 2);
    let error = validate_blake3_hex(&hash).expect_err("non-ASCII hex is invalid");

    assert_eq!(error.reason(), "expected lowercase hex characters only");
}
