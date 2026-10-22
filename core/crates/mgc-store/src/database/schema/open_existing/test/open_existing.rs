//! Existing writable opens must never create missing files or follow final symlinks.
//! Mở ghi DB có sẵn không được tạo file thiếu hoặc follow symlink cuối.

use crate::database::Database;
#[cfg(any(unix, windows))]
use std::path::Path;

#[test]
fn open_existing_does_not_create_a_missing_database() {
    let temp = tempfile::tempdir().unwrap();
    let database_path = temp.path().join("missing.db");

    assert!(Database::open_existing(&database_path).is_err());
    assert!(!database_path.exists());
}

#[test]
fn open_existing_initializes_an_existing_database_file() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let database_path = root.join("existing.db");
    drop(rusqlite::Connection::open(&database_path).unwrap());

    let database = Database::open_existing(&database_path).unwrap();

    assert!(database.list_installed().unwrap().is_empty());
    assert!(database_path.is_file());
}

#[cfg(any(unix, windows))]
#[test]
fn open_existing_rejects_final_component_symlink() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target.db");
    drop(Database::open(&target).unwrap());
    let link = temp.path().join("link.db");
    create_file_symlink(&target, &link).unwrap();

    assert!(Database::open_existing(&link).is_err());
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
