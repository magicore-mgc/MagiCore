//! Atomic replacement tests — Test replace atomic.
//! The platform driver must publish the complete source — Driver theo OS phải publish đủ source.

use super::atomic_replace_file;
use std::fs;

#[test]
fn replacement_publishes_source_and_consumes_it() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let source = directory.path().join("temporary.json");
    let destination = directory.path().join("keyring.json");
    fs::write(&source, b"new keyring").expect("test operation must succeed");
    fs::write(&destination, b"old keyring").expect("test operation must succeed");

    atomic_replace_file(&source, &destination).expect("test operation must succeed");

    assert_eq!(
        fs::read(&destination).expect("test operation must succeed"),
        b"new keyring"
    );
    assert!(!source.exists());
}
