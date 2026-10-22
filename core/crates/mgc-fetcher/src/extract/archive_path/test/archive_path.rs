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
