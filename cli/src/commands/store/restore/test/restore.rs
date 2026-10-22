//! End-to-end source cases for restore ordering and partial backup semantics.
//! Ca source end-to-end cho thứ tự restore và semantics backup từng phần.

use super::super::restore_inner;
use std::fs;

#[test]
fn database_only_backup_leaves_project_virtual_store_untouched() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let project_vstore = project.join("node_modules/.magicore");
    fs::create_dir_all(&project_vstore).unwrap();
    fs::write(project_vstore.join("keep.bin"), b"keep").unwrap();
    let project_db = project.join(".magicore/cache/web/store.db");
    fs::create_dir_all(project_db.parent().unwrap()).unwrap();
    fs::write(&project_db, b"old database").unwrap();
    let backup = temp.path().join("backup");
    fs::create_dir(&backup).unwrap();
    fs::write(backup.join("store.db"), b"new database").unwrap();
    let safety = temp.path().join("safety");

    restore_inner(&project, &backup, Some(&safety)).unwrap();

    assert_eq!(fs::read(project_db).unwrap(), b"new database");
    assert_eq!(fs::read(project_vstore.join("keep.bin")).unwrap(), b"keep");
    assert_eq!(fs::read(safety.join("vstore/keep.bin")).unwrap(), b"keep");
}

#[test]
fn virtual_store_only_backup_leaves_project_database_untouched() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let project_db = project.join(".magicore/cache/web/store.db");
    fs::create_dir_all(project_db.parent().unwrap()).unwrap();
    fs::write(&project_db, b"current database").unwrap();
    let backup = temp.path().join("backup");
    fs::create_dir_all(backup.join("vstore")).unwrap();
    fs::write(backup.join("vstore/package.bin"), b"restored package").unwrap();
    let safety = temp.path().join("safety");

    restore_inner(&project, &backup, Some(&safety)).unwrap();

    assert_eq!(fs::read(project_db).unwrap(), b"current database");
    assert_eq!(
        fs::read(project.join("node_modules/.magicore/package.bin")).unwrap(),
        b"restored package"
    );
    assert_eq!(
        fs::read(safety.join("store.db")).unwrap(),
        b"current database"
    );
}

#[test]
fn active_web_project_lock_blocks_restore_before_project_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let project_root = temp.path().join("project");
    let current_vstore = project_root.join("node_modules/.magicore");
    fs::create_dir_all(&current_vstore).unwrap();
    fs::write(current_vstore.join("current.bin"), b"current").unwrap();

    let backup = temp.path().join("backup");
    fs::create_dir_all(backup.join("vstore")).unwrap();
    fs::write(backup.join("store.db"), b"backup database").unwrap();
    fs::write(backup.join("vstore/backup.bin"), b"backup").unwrap();

    let _web_install_lock =
        crate::commands::store::acquire_maintenance_lock(&project_root).unwrap();

    let safety = temp.path().join("safety");
    assert!(restore_inner(&project_root, &backup, Some(&safety)).is_err());
    assert_eq!(
        fs::read(current_vstore.join("current.bin")).unwrap(),
        b"current"
    );
    assert!(!current_vstore.join("backup.bin").exists());
    assert!(!safety.exists());
}
