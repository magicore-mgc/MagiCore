//! Checks replacement through a path beyond the legacy MAX_PATH length.
//! Kiểm tra thay file qua path dài hơn giới hạn MAX_PATH cũ.

use super::super::windows_replace;
use tempfile::tempdir;

#[test]
fn replaces_files_under_an_extended_windows_path() {
    let project = tempdir().unwrap();
    let mut directory = std::fs::canonicalize(project.path()).unwrap();
    for _ in 0..8 {
        directory = directory.join("package-store-long-component-1234567890");
    }
    std::fs::create_dir_all(&directory).unwrap();
    let staging = directory.join("staging.tmp");
    let destination = directory.join("package-marker.json");
    std::fs::write(&staging, b"first").unwrap();
    windows_replace(&staging, &destination, true).unwrap();
    assert_eq!(std::fs::read(&destination).unwrap(), b"first");
    std::fs::write(&staging, b"second").unwrap();
    windows_replace(&staging, &destination, true).unwrap();
    assert_eq!(std::fs::read(&destination).unwrap(), b"second");
    assert!(!staging.exists());
}
