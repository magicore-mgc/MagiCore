//! Database restore cases.
//! Ca khôi phục database.

use super::super::super::error::DatabaseRestoreError;
use super::super::restore_database;
use std::fs;

#[test]
fn publishes_database_bytes_to_project_path() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("backup/store.db");
    let destination = temp.path().join("project/.magicore/cache/web/store.db");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(&source, b"restored database").unwrap();

    restore_database(&source, &destination).unwrap();

    assert_eq!(fs::read(destination).unwrap(), b"restored database");
}

#[cfg(unix)]
#[test]
fn refuses_symlink_database_destination_without_overwriting_target() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.db");
    let destination = temp.path().join("project/store.db");
    let outside = temp.path().join("outside.db");
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(&source, b"new").unwrap();
    fs::write(&outside, b"keep").unwrap();
    symlink(&outside, &destination).unwrap();

    assert!(matches!(
        restore_database(&source, &destination),
        Err(DatabaseRestoreError::Copy { .. })
    ));
    assert_eq!(fs::read(outside).unwrap(), b"keep");
}
