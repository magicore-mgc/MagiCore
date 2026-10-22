//! Tests safe extraction success and parent-traversal rejection.
//! Kiểm thử giải nén an toàn và từ chối đường dẫn traversal.

use super::super::super::test_support::{tar_gz_file, tar_gz_file_with_raw_path};
use crate::protocols::archive::extract_tar_gz;

#[test]
fn extracts_regular_files_under_the_destination() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let archive = tar_gz_file("nested/file.txt", b"content");

    extract_tar_gz(&archive, root.path()).expect("test operation must succeed");

    assert_eq!(
        std::fs::read(root.path().join("nested/file.txt")).expect("test operation must succeed"),
        b"content"
    );
}

#[test]
fn rejects_parent_traversal_without_writing_outside_destination() {
    let parent = tempfile::tempdir().expect("test operation must succeed");
    let destination = parent.path().join("extract");
    let escaped = parent.path().join("escaped.txt");
    let archive = tar_gz_file_with_raw_path("../escaped.txt", b"outside");

    assert!(extract_tar_gz(&archive, &destination).is_err());
    assert!(!escaped.exists());
}
