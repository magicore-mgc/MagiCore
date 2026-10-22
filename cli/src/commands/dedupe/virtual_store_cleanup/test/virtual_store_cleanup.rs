//! Virtual-store cleanup tests — verifies lock retention and successful-delete bytes.
//! Test dọn virtual store — xác minh giữ package còn lock và byte xóa thành công.
use super::super::{cleanup_unreferenced_vstore, vstore_root};
use mgc_lockfile::{Lockfile, Package};
use std::fs;

#[test]
fn cleanup_removes_only_unreferenced_dirs_and_counts_confirmed_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let store = vstore_root(root);
    let kept = store.join("kept@1.0.0");
    let orphan = store.join("orphan@2.0.0");
    fs::create_dir_all(&kept).unwrap();
    fs::create_dir_all(&orphan).unwrap();
    fs::write(orphan.join("payload.bin"), b"four").unwrap();

    let mut lock = Lockfile::new();
    lock.packages.push(Package {
        name: "kept".to_string(),
        version: "1.0.0".to_string(),
        ..Default::default()
    });

    let report = cleanup_unreferenced_vstore(root, &lock);

    assert!(kept.is_dir());
    assert!(!orphan.exists());
    assert_eq!(report.deleted_count(), 1);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 4);
    assert_eq!(report.failed_count(), 0);
}
