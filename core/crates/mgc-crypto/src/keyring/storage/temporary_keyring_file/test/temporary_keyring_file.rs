//! Temporary keyring-file guard tests — Test guard file keyring tạm.
//! Drop cleans uncommitted temporary paths — Drop dọn path tạm chưa commit.

use super::TemporaryKeyringFile;
use std::fs;

#[test]
fn dropping_armed_guard_removes_temporary_file() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("keyring.tmp");
    fs::write(&path, b"temporary").expect("test operation must succeed");
    drop(TemporaryKeyringFile::new(path.clone()));
    assert!(!path.exists());
}

#[test]
fn disarmed_guard_keeps_published_file() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("keyring.json");
    fs::write(&path, b"published").expect("test operation must succeed");
    let mut guard = TemporaryKeyringFile::new(path.clone());
    guard.disarm();
    drop(guard);
    assert_eq!(
        fs::read(&path).expect("test operation must succeed"),
        b"published"
    );
}
