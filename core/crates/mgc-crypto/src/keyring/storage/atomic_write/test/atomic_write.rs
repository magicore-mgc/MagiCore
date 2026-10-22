//! Atomic writer tests — Test writer atomic.
//! Exercise the production writer helper — Gọi helper writer production.

use super::write_private_file_atomic;
use std::fs;

#[test]
fn writes_complete_contents_and_removes_temporary_entries() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("keyring.json");
    write_private_file_atomic(&path, b"atomic contents").expect("test operation must succeed");
    assert_eq!(
        fs::read(&path).expect("test operation must succeed"),
        b"atomic contents"
    );
    assert!(
        directory
            .path()
            .read_dir()
            .expect("test operation must succeed")
            .all(|entry| {
                !entry
                    .expect("test operation must succeed")
                    .file_name()
                    .to_string_lossy()
                    .ends_with(".tmp")
            })
    );
}
