//! Adler-32 owner tests — Test owner Adler-32.
//! Check the reference vector and empty input — Kiểm tra vector chuẩn và đầu vào rỗng.

use crate::checksum::checksum_adler32;

#[test]
fn matches_reference_vector_and_empty_input() {
    assert_eq!(checksum_adler32(b"hello"), 0x062c0215);
    assert_eq!(checksum_adler32(b""), 1);
}
