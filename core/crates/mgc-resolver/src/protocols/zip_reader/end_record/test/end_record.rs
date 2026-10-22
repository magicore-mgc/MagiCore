//! EOCD malformed-input regression — regression input EOCD lỗi.

use crate::protocols::zip_reader::read_zip_entries;

#[test]
fn zip_truncated_rejected() {
    assert!(read_zip_entries(b"PK\x03\x04short").is_err());
    assert!(read_zip_entries(b"").is_err());
}
