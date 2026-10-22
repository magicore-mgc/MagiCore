// Chunk-transfer tests — Content-Range boundary construction.
// Test truyền chunk — kiểm tra dựng Content-Range tại biên.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::content_range_value;

#[test]
fn content_range_uses_inclusive_end() {
    assert_eq!(content_range_value(0, 1).unwrap(), "0-0");
    assert_eq!(content_range_value(5, 4).unwrap(), "5-8");
}

#[test]
fn content_range_rejects_offset_overflow() {
    assert_eq!(
        content_range_value(u64::MAX, 1).unwrap_err().to_string(),
        "OCI blob upload offset overflow"
    );
}
