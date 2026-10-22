//! Checks signature-sidecar derivation and absence handling.
//! Kiểm tra tạo path sidecar và xử lý file vắng mặt.

use super::super::{signature_file_presence, signature_path_for};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn signature_path_replaces_the_lock_extension() {
    assert_eq!(
        signature_path_for(Path::new("mgc.lock")),
        Path::new("mgc.lock.sig").to_path_buf()
    );
}

#[test]
fn missing_sidecar_is_distinct_from_present_sidecar() {
    let directory = tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    assert!(!signature_file_presence(&lock_path).expect("test operation must succeed"));
    std::fs::write(lock_path.with_extension("lock.sig"), b"sidecar")
        .expect("test operation must succeed");
    assert!(signature_file_presence(&lock_path).expect("test operation must succeed"));
}
