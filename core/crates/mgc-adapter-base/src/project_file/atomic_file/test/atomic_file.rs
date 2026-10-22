//! Atomic replacement of an existing regular file.
//! Kiểm tra thay thế atomic trên file thường đã có.

use super::atomic_write_regular;
use std::fs;

#[test]
fn replaces_contents_of_an_existing_regular_file() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("manifest.toml");
    fs::write(&path, "before\n").unwrap();

    atomic_write_regular(&path, b"after\n").unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), "after\n");
}
