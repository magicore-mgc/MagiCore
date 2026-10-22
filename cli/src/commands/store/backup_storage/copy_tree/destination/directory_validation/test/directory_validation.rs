//! Tests for child containment and empty destination validation.
//! Kiểm thử containment directory con và xác thực destination rỗng.

#![allow(clippy::unwrap_used)]

use super::super::super::error::DestinationError;
use super::super::implementation::{
    ensure_destination_directory, ensure_empty_destination_directory,
};

#[test]
fn creates_a_child_inside_the_destination_root() {
    let temp = tempfile::tempdir().unwrap();
    let root_path = temp.path().join("root");
    std::fs::create_dir(&root_path).unwrap();
    let root = std::fs::canonicalize(root_path).unwrap();
    let child = root.join("nested");

    ensure_destination_directory(&child, &root).unwrap();

    assert!(child.is_dir());
    assert!(std::fs::canonicalize(&child).unwrap().starts_with(&root));
}

#[test]
fn refuses_a_directory_outside_the_destination_root() {
    let temp = tempfile::tempdir().unwrap();
    let root_path = temp.path().join("root");
    let outside = temp.path().join("outside");
    std::fs::create_dir(&root_path).unwrap();
    let root = std::fs::canonicalize(root_path).unwrap();

    assert!(matches!(
        ensure_destination_directory(&outside, &root),
        Err(DestinationError::DirectoryEscapesRoot { .. })
    ));
}

#[cfg(unix)]
#[test]
fn refuses_a_symlink_child_directory() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let root_path = temp.path().join("root");
    let target = temp.path().join("target");
    std::fs::create_dir(&root_path).unwrap();
    let root = std::fs::canonicalize(root_path).unwrap();
    let child = root.join("child-link");
    std::fs::create_dir(&target).unwrap();
    symlink(&target, &child).unwrap();

    assert!(matches!(
        ensure_destination_directory(&child, &root),
        Err(DestinationError::SymlinkDirectory(_))
    ));
}

#[test]
fn accepts_only_empty_real_destinations() {
    let temp = tempfile::tempdir().unwrap();
    ensure_empty_destination_directory(temp.path()).unwrap();
    std::fs::write(temp.path().join("entry"), "data").unwrap();

    assert!(matches!(
        ensure_empty_destination_directory(temp.path()),
        Err(DestinationError::DestinationNotEmpty(_))
    ));
}
