//! Verify duplicate and root-metadata archive indexing rules.
//! Kiểm tra quy tắc index duplicate và metadata root.

use super::super::index_wheel_entries;
use crate::protocols::zip_reader::ZipEntry;

#[test]
fn archive_index_rejects_duplicate_members() {
    let entries = vec![
        ZipEntry {
            name: "pkg/file.py".to_string(),
            data: b"one".to_vec(),
        },
        ZipEntry {
            name: "pkg/file.py".to_string(),
            data: b"two".to_vec(),
        },
    ];

    let error = index_wheel_entries(entries).expect_err("test operation must fail");
    assert!(error.to_string().contains("duplicate member"));
}

#[test]
fn archive_index_selects_only_root_dist_info() {
    let entries = vec![
        ZipEntry {
            name: "demo-1.0.dist-info/RECORD".to_string(),
            data: b"root record".to_vec(),
        },
        ZipEntry {
            name: "demo-1.0.dist-info/METADATA".to_string(),
            data: b"Name: demo\nVersion: 1.0\n".to_vec(),
        },
        ZipEntry {
            name: "demo/vendor/other.dist-info/RECORD".to_string(),
            data: b"vendor data".to_vec(),
        },
    ];

    let archive = index_wheel_entries(entries).expect("test operation must succeed");
    assert_eq!(archive.record_path(), Some("demo-1.0.dist-info/RECORD"));
    assert_eq!(archive.metadata_path(), Some("demo-1.0.dist-info/METADATA"));
    assert_eq!(archive.member_count(), 3);
}
