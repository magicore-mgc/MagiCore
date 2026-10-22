//! ZIP read pipeline regression — regression pipeline đọc ZIP.

use crate::protocols::zip_reader::{read_zip_entries, test_support::build_zip};

#[test]
fn zip_store_and_deflate_roundtrip() {
    let zip = build_zip(&[
        ("dir/", Vec::new(), 0), // Directory entry is skipped — bỏ qua entry thư mục.
        ("a.txt", b"hello zip".to_vec(), 0),
        ("nested/b.bin", b"deflated payload 1234567890".to_vec(), 8),
    ]);
    let entries = read_zip_entries(&zip).expect("test operation must succeed");
    assert_eq!(entries.len(), 2, "directory entry must be skipped");
    assert_eq!(entries[0].name, "a.txt");
    assert_eq!(entries[0].data, b"hello zip");
    assert_eq!(entries[1].name, "nested/b.bin");
    assert_eq!(entries[1].data, b"deflated payload 1234567890");
}
