//! Read-only access must reject missing/old stores and deny SQL writes.
//! Truy cập chỉ đọc phải chặn DB thiếu/schema cũ và từ chối SQL ghi.

use crate::database::Database;
use mgc_types::PackageId;
use std::path::Path;

#[test]
fn readonly_open_does_not_create_a_missing_database() {
    let temp = tempfile::tempdir().unwrap();
    let database_path = temp.path().join("missing.db");

    assert!(Database::open_read_only(&database_path).is_err());
    assert!(!database_path.exists());
}

#[test]
fn readonly_open_rejects_an_old_schema_without_migrating_it() {
    let temp = tempfile::tempdir().unwrap();
    let database_path = temp.path().join("old.db");
    drop(rusqlite::Connection::open(&database_path).unwrap());
    let before = std::fs::read(&database_path).unwrap();

    assert!(Database::open_read_only(&database_path).is_err());
    assert_eq!(std::fs::read(&database_path).unwrap(), before);
}

#[test]
fn readonly_connection_rejects_mutating_database_operations() {
    let temp = tempfile::tempdir().unwrap();
    let database_path = temp.path().join("store.db");
    let package = PackageId::parse("readonly-probe@1.0.0").unwrap();
    drop(Database::open(&database_path).unwrap());

    let readonly = Database::open_read_only(&database_path).unwrap();

    assert!(readonly.insert_package(&package, None).is_err());
    assert!(!readonly.is_installed(&package).unwrap());
}

#[cfg(any(unix, windows))]
#[test]
fn readonly_open_rejects_final_component_symlinks() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target.db");
    drop(Database::open(&target).unwrap());
    let link = temp.path().join("link.db");
    create_file_symlink(&target, &link).unwrap();

    assert!(Database::open_read_only(&link).is_err());
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
