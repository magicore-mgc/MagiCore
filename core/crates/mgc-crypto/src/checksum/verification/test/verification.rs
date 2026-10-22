//! Checksum verification tests — Test xác minh checksum.
//! Preserve the match and mismatch result contract — Giữ contract kết quả khớp và không khớp.

use crate::checksum::verify_adler32;

#[test]
fn reports_match_and_mismatch() {
    assert!(verify_adler32(b"hello", 0x062c0215).expect("test operation must succeed"));
    assert!(!verify_adler32(b"hello", 0).expect("test operation must succeed"));
}
