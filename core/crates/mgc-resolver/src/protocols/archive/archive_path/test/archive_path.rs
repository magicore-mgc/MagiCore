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

#[test]
fn rejects_windows_unsafe_archive_paths() {
    // NTFS alternate data stream, trailing dot/space, and reserved device names
    // resolve to a different file on Windows — all must be rejected.
    // NTFS ADS, dấu chấm/khoảng trắng cuối, và tên thiết bị dành riêng trỏ tới
    // file khác trên Windows — đều phải bị từ chối.
    for unsafe_path in [
        "pkg/file.txt:ads",
        "pkg/file:stream",
        "pkg/file.",
        "pkg/file ",
        "CON",
        "pkg/nul.txt",
        "pkg/COM1",
        "pkg/LPT9.log",
    ] {
        assert!(
            sanitize_archive_path(Path::new(unsafe_path)).is_err(),
            "expected {unsafe_path:?} to be rejected"
        );
    }
}

#[test]
fn accepts_ordinary_names_that_look_windows_special() {
    // "console.log" and "com10" are ordinary names, not reserved devices.
    // "console.log" và "com10" là tên thường, không phải thiết bị dành riêng.
    for safe_path in ["pkg/console.log", "pkg/com10.txt", "pkg/nullify.txt"] {
        assert!(
            sanitize_archive_path(Path::new(safe_path)).is_ok(),
            "expected {safe_path:?} to be accepted"
        );
    }
}
