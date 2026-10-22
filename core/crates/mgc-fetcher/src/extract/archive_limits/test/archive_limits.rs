//! Keeps the configured entry and uncompressed-byte budgets explicit.
//! Kiểm tra rõ giới hạn số entry và tổng byte chưa nén.

#[test]
fn preserves_archive_expansion_limits() {
    assert_eq!(super::super::MAX_ARCHIVE_ENTRIES, 16_384);
    assert_eq!(super::super::MAX_ENTRY_BYTES, 256 * 1024 * 1024);
    assert_eq!(super::super::MAX_TOTAL_FILE_BYTES, 512 * 1024 * 1024);
}
