//! Safe extraction regressions — regression giải nén an toàn.

use crate::protocols::zip_reader::{extract_zip, test_support::build_zip};

#[test]
fn zip_extract_writes_files_and_blocks_traversal() {
    let good = build_zip(&[("pkg/lib/file.txt", b"content".to_vec(), 8)]);
    let dir = tempfile::tempdir().expect("test operation must succeed");
    extract_zip(&good, dir.path()).expect("test operation must succeed");
    let written =
        std::fs::read(dir.path().join("pkg/lib/file.txt")).expect("test operation must succeed");
    assert_eq!(written, b"content");

    let evil = build_zip(&[("../escape.txt", b"x".to_vec(), 0)]);
    let err = extract_zip(&evil, dir.path()).expect_err("test operation must fail");
    assert!(err.to_string().contains("unsafe zip entry path"), "{err}");
}

#[cfg(unix)]
#[test]
fn zip_extract_rejects_preexisting_symlink_parent() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("test operation must succeed");
    let outside = tempfile::tempdir().expect("test operation must succeed");
    symlink(outside.path(), root.path().join("nested")).expect("test operation must succeed");
    let archive = build_zip(&[("nested/pwned.txt", b"outside".to_vec(), 0)]);

    let result = extract_zip(&archive, root.path());

    assert!(result.is_err(), "extractor must refuse a linked parent");
    assert!(
        !outside.path().join("pwned.txt").exists(),
        "archive data must never be written through a pre-existing symlink"
    );
}

#[cfg(unix)]
#[test]
fn zip_extract_rejects_symlink_destination_and_file_target() {
    use std::os::unix::fs::symlink;

    let parent = tempfile::tempdir().expect("test operation must succeed");
    let outside = tempfile::tempdir().expect("test operation must succeed");
    let linked_root = parent.path().join("linked-root");
    symlink(outside.path(), &linked_root).expect("test operation must succeed");
    let root_archive = build_zip(&[("pwned.txt", b"root escape".to_vec(), 0)]);
    assert!(extract_zip(&root_archive, &linked_root).is_err());
    assert!(!outside.path().join("pwned.txt").exists());

    let root = tempfile::tempdir().expect("test operation must succeed");
    let linked_file = outside.path().join("existing.txt");
    std::fs::write(&linked_file, b"original").expect("test operation must succeed");
    symlink(&linked_file, root.path().join("target.txt")).expect("test operation must succeed");
    let target_archive = build_zip(&[("target.txt", b"overwrite".to_vec(), 0)]);
    assert!(extract_zip(&target_archive, root.path()).is_err());
    assert_eq!(
        std::fs::read(&linked_file).expect("test operation must succeed"),
        b"original"
    );
}
