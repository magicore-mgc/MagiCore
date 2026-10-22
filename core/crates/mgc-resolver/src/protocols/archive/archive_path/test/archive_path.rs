//! Tests archive path normalization and traversal rejection.
//! Kiểm thử chuẩn hóa path archive và từ chối traversal.

use super::super::*;
use std::path::{Path, PathBuf};
#[test]
fn rejects_empty_parent_and_rooted_archive_paths() {
    for path in [Path::new(""), Path::new("../escape"), Path::new("/")] {
        assert!(sanitize_archive_path(path).is_err());
    }
}

#[test]
fn keeps_safe_relative_archive_paths() {
    assert_eq!(
        sanitize_archive_path(Path::new("package/src/lib.rs"))
            .expect("test operation must succeed"),
        PathBuf::from("package/src/lib.rs")
    );
}
