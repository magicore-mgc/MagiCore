//! Tests for non-creating destination path projection.
//! Kiểm thử chiếu path destination mà không tạo thư mục.

#![allow(clippy::unwrap_used)]

use super::super::implementation::prospective_destination_root;

#[test]
fn projects_missing_components_without_creating_them() {
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("new/nested/backup");
    let expected = std::fs::canonicalize(temp.path())
        .unwrap()
        .join("new/nested/backup");

    let projected = prospective_destination_root(&destination).unwrap();

    assert_eq!(projected, expected);
    assert!(!destination.exists());
}

#[test]
fn canonicalizes_an_existing_destination_directory() {
    let temp = tempfile::tempdir().unwrap();

    let projected = prospective_destination_root(temp.path()).unwrap();

    assert_eq!(projected, std::fs::canonicalize(temp.path()).unwrap());
}

#[cfg(unix)]
#[test]
fn projects_a_final_symlink_as_a_link_path() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("link");
    std::fs::create_dir(&target).unwrap();
    symlink(&target, &link).unwrap();
    let expected = std::fs::canonicalize(temp.path()).unwrap().join("link");

    assert_eq!(prospective_destination_root(&link).unwrap(), expected);
}
