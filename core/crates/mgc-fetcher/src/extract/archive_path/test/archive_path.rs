//! Covers traversal rejection and harmless current-directory components.
//! Kiểm tra từ chối traversal và xử lý thành phần current-directory.

use std::path::Path;

#[test]
fn rejects_parent_and_empty_archive_paths() {
    assert!(matches!(
        super::super::sanitize_archive_path(Path::new("../escape")),
        Err(super::super::super::ExtractionError::UnsafeArchivePath(_))
    ));
    assert!(matches!(
        super::super::sanitize_archive_path(Path::new("./")),
        Err(super::super::super::ExtractionError::EmptyArchivePath)
    ));
}

#[test]
fn removes_current_directory_components() {
    let path = super::super::sanitize_archive_path(Path::new("./pkg/./file")).unwrap();
    assert_eq!(path, Path::new("pkg/file"));
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
            matches!(
                super::super::sanitize_archive_path(Path::new(unsafe_path)),
                Err(super::super::super::ExtractionError::UnsafeArchivePath(_))
            ),
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
            super::super::sanitize_archive_path(Path::new(safe_path)).is_ok(),
            "expected {safe_path:?} to be accepted"
        );
    }
}
