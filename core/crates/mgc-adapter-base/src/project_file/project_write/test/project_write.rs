//! Unix dirfd-based atomic project write regression.
//! Kiểm tra ghi atomic qua dirfd trên Unix.

use super::atomic_write_project_regular_dirfd;
use std::fs;
use std::path::Path;

#[test]
fn atomically_replaces_a_nested_regular_file() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("nested")).unwrap();
    fs::write(root.path().join("nested/settings.toml"), "before\n").unwrap();

    atomic_write_project_regular_dirfd(root.path(), Path::new("nested/settings.toml"), b"after\n")
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.path().join("nested/settings.toml")).unwrap(),
        "after\n"
    );
}
