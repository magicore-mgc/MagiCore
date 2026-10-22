//! Write-if-changed contract tests.
//! Test contract chỉ ghi khi khác.

use super::super::atomic_write_if_changed;

#[test]
fn identical_bytes_are_not_rewritten() {
    let dir = tempfile::tempdir().expect("temporary directory");
    let path = dir.path().join("package.json");
    std::fs::write(&path, b"same").expect("write initial bytes");

    assert!(!atomic_write_if_changed(&path, b"same").expect("compare identical bytes"));
    assert_eq!(std::fs::read(path).expect("read file"), b"same");
}

#[test]
fn changed_bytes_are_written() {
    let dir = tempfile::tempdir().expect("temporary directory");
    let path = dir.path().join("package.json");
    std::fs::write(&path, b"before").expect("write initial bytes");

    assert!(atomic_write_if_changed(&path, b"after").expect("write changed bytes"));
    assert_eq!(std::fs::read(path).expect("read file"), b"after");
}
