//! Verifies the stable profile labels and value order.
//! Xác minh nhãn profile và thứ tự giá trị ổn định.

#[test]
fn preserves_profile_output_contract() {
    assert_eq!(
        super::super::format_profile_line(3, 4, 5, 6, 7),
        "[magicore:fetcher-profile] files=3 archive_read_ms=4 cas_import_batch_ms=5 file_export_batch_ms=6 total_ms=7"
    );
}
