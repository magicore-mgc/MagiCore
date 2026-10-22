//! Backup source validation cases.
//! Ca xác thực nguồn backup.

use super::super::super::error::BackupValidationError;
use super::super::validate_backup;
use std::fs;

#[test]
fn rejects_empty_backup_and_accepts_database_only_backup() {
    let empty = tempfile::tempdir().unwrap();
    assert!(matches!(
        validate_backup(empty.path()),
        Err(BackupValidationError::EmptyBackup(_))
    ));

    fs::write(empty.path().join("store.db"), b"db").unwrap();
    let partial = validate_backup(empty.path()).unwrap();
    assert!(partial.database.is_some());
    assert!(partial.virtual_store.is_none());
}

#[cfg(unix)]
#[test]
fn rejects_database_symlink() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let outside = root.path().join("outside.db");
    fs::write(&outside, b"db").unwrap();
    symlink(&outside, root.path().join("store.db")).unwrap();
    assert!(matches!(
        validate_backup(root.path()),
        Err(BackupValidationError::SymlinkEntry(_))
    ));
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_vstore_root() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), root.path().join("vstore")).unwrap();
    assert!(matches!(
        validate_backup(root.path()),
        Err(BackupValidationError::SymlinkEntry(_))
    ));
}

#[cfg(unix)]
#[test]
fn rejects_external_link_inside_vstore_before_accepting_backup() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("vstore")).unwrap();
    fs::write(outside.path().join("payload"), b"outside").unwrap();
    symlink(
        outside.path().join("payload"),
        root.path().join("vstore/external"),
    )
    .unwrap();
    assert!(validate_backup(root.path()).is_err());
}
