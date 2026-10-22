//! Store pruning must reject unsafe roots and database files before opening SQLite.
//! Prune store phải chặn root/DB không an toàn trước khi mở SQLite.

use super::super::error::RootSafetyError;
use super::super::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
    validate_existing_lock_directory,
};
use std::fs;

#[test]
fn missing_project_store_is_a_noop_root() {
    let temp = tempfile::tempdir().unwrap();
    assert!(existing_store_cache_root(temp.path()).unwrap().is_none());
    assert!(existing_vstore_root(temp.path()).unwrap().is_none());
}

#[test]
fn missing_database_is_not_opened_or_created_by_path_owner() {
    let temp = tempfile::tempdir().unwrap();
    let cache_root = temp.path().join(".magicore/cache/web");
    fs::create_dir_all(&cache_root).unwrap();

    assert!(existing_database_file(&cache_root).unwrap().is_none());
    assert!(!cache_root.join("store.db").exists());
}

#[cfg(any(unix, windows))]
#[test]
fn database_symlink_is_rejected_before_open() {
    use super::symlink_support::symlink_file;

    let temp = tempfile::tempdir().unwrap();
    let cache_root = temp.path().join(".magicore/cache/web");
    fs::create_dir_all(&cache_root).unwrap();
    let external = temp.path().join("external.db");
    fs::write(&external, b"external").unwrap();
    symlink_file(&external, &cache_root.join("store.db")).unwrap();

    assert!(matches!(
        existing_database_file(&cache_root),
        Err(RootSafetyError::SymlinkedDatabase { .. })
    ));
    assert_eq!(fs::read(external).unwrap(), b"external");
}

#[cfg(any(unix, windows))]
#[test]
fn lock_directory_symlink_is_rejected_before_lock_acquisition() {
    use super::symlink_support::symlink_directory;

    let temp = tempfile::tempdir().unwrap();
    let cache_root = temp.path().join(".magicore/cache/web");
    let external = temp.path().join("external-locks");
    fs::create_dir_all(&cache_root).unwrap();
    fs::create_dir(&external).unwrap();
    symlink_directory(&external, &cache_root.join("locks")).unwrap();

    assert!(matches!(
        validate_existing_lock_directory(&cache_root),
        Err(RootSafetyError::Filesystem(
            crate::commands::safe_filesystem::SafeFilesystemError::SymlinkedDirectory { .. }
        ))
    ));
}
