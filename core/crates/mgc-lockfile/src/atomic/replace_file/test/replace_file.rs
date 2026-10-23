//! Verifies replacement of an existing destination and staging consumption.
//! Kiểm tra thay destination đang có và staging file được tiêu thụ.

use super::super::atomic_replace_file;
use tempfile::tempdir;

#[test]
fn replacement_publishes_new_bytes_and_consumes_staging_path() {
    let directory = tempdir().expect("test operation must succeed");
    let staging = directory.path().join("staging.tmp");
    let destination = directory.path().join("mgc.lock");
    std::fs::write(&staging, b"new lock").expect("test operation must succeed");
    std::fs::write(&destination, b"old lock").expect("test operation must succeed");

    atomic_replace_file(&staging, &destination).expect("test operation must succeed");

    assert_eq!(
        std::fs::read(&destination).expect("test operation must succeed"),
        b"new lock"
    );
    assert!(!staging.exists());
}
