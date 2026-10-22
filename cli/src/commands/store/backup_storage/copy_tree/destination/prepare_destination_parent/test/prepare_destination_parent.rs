//! Tests for existing and missing destination parent preparation.
//! Kiểm thử chuẩn bị parent destination đã có và còn thiếu.

#![allow(clippy::unwrap_used)]

use super::super::super::error::DestinationError;
use super::super::implementation::prepare_destination_parent;

#[test]
fn creates_a_missing_parent_and_returns_its_canonical_path() {
    let temp = tempfile::tempdir().unwrap();
    let parent = temp.path().join("nested/parent");

    let canonical = prepare_destination_parent(&parent).unwrap();

    assert!(parent.is_dir());
    assert_eq!(std::fs::canonicalize(&parent).unwrap(), canonical);
}

#[test]
fn rejects_a_regular_file_as_parent() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("parent-file");
    std::fs::write(&file, "data").unwrap();

    assert!(matches!(
        prepare_destination_parent(&file),
        Err(DestinationError::NotDirectory(_))
    ));
}

#[cfg(unix)]
#[test]
fn canonicalizes_a_symlink_parent() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("parent-link");
    std::fs::create_dir(&target).unwrap();
    symlink(&target, &link).unwrap();

    assert_eq!(
        prepare_destination_parent(&link).unwrap(),
        std::fs::canonicalize(&target).unwrap()
    );
}
