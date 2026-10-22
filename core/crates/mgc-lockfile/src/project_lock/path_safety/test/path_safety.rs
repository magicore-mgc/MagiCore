//! Checks the public link predicate and directory type rejection.
//! Kiểm tra predicate link công khai và từ chối sai loại directory.

use super::super::{PathSafetyError, ensure_real_dir, path_is_link_or_reparse};
use tempfile::tempdir;

#[test]
fn missing_path_is_not_reported_as_a_link() {
    let directory = tempdir().expect("test operation must succeed");
    assert!(!path_is_link_or_reparse(&directory.path().join("missing")));
}

#[test]
fn a_regular_file_cannot_be_used_as_magicore_directory() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join(".magicore");
    std::fs::write(&path, b"file").expect("test operation must succeed");
    assert!(matches!(
        ensure_real_dir(&path),
        Err(PathSafetyError::NotDirectory { .. })
    ));
}

#[cfg(unix)]
#[test]
fn symlinked_path_is_reported_as_a_link() {
    let directory = tempdir().expect("test operation must succeed");
    let target = directory.path().join("target");
    let link = directory.path().join("link");
    std::fs::create_dir(&target).expect("test operation must succeed");
    std::os::unix::fs::symlink(&target, &link).expect("test operation must succeed");
    assert!(path_is_link_or_reparse(&link));
}
