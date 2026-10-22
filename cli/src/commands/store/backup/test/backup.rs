//! Project Web-store backup cases.
//! Ca backup Web-store project.

use super::super::backup_inner;
use std::fs;

#[test]
fn copies_database_and_virtual_store_payload() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let database = project.join(".magicore/cache/web/store.db");
    let package = project.join("node_modules/.magicore/pkg/package.json");
    fs::create_dir_all(database.parent().unwrap()).unwrap();
    fs::create_dir_all(package.parent().unwrap()).unwrap();
    fs::write(&database, b"db bytes").unwrap();
    fs::write(&package, b"package bytes").unwrap();
    let destination = temp.path().join("backup");

    backup_inner(&project, Some(destination.display().to_string())).unwrap();

    assert_eq!(fs::read(destination.join("store.db")).unwrap(), b"db bytes");
    assert_eq!(
        fs::read(destination.join("vstore/pkg/package.json")).unwrap(),
        b"package bytes"
    );
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_backup_root_before_writing_to_its_target() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let package = project.join("node_modules/.magicore/pkg/package.json");
    fs::create_dir_all(package.parent().unwrap()).unwrap();
    fs::write(&package, b"package bytes").unwrap();
    let target = temp.path().join("external");
    fs::create_dir(&target).unwrap();
    let destination = temp.path().join("backup-link");
    symlink(&target, &destination).unwrap();

    assert!(backup_inner(&project, Some(destination.display().to_string())).is_err());
    assert_eq!(fs::read_dir(target).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn refuses_symlinked_project_database_before_creating_backup() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let database = project.join(".magicore/cache/web/store.db");
    fs::create_dir_all(database.parent().unwrap()).unwrap();
    let outside = temp.path().join("outside.db");
    fs::write(&outside, b"external database").unwrap();
    symlink(&outside, &database).unwrap();
    let destination = temp.path().join("backup");

    assert!(backup_inner(&project, Some(destination.display().to_string())).is_err());
    assert!(!destination.exists());
    assert_eq!(fs::read(outside).unwrap(), b"external database");
}

#[test]
fn active_project_install_lock_blocks_backup_before_destination_creation() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let destination = temp.path().join("backup");
    let _install_lock = crate::commands::store::acquire_maintenance_lock(&project).unwrap();

    assert!(backup_inner(&project, Some(destination.display().to_string())).is_err());
    assert!(!destination.exists());
}

#[test]
fn refuses_non_empty_destination_without_mixing_snapshots() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let database = project.join(".magicore/cache/web/store.db");
    let package = project.join("node_modules/.magicore/pkg/package.json");
    fs::create_dir_all(database.parent().unwrap()).unwrap();
    fs::create_dir_all(package.parent().unwrap()).unwrap();
    fs::write(&database, b"new database").unwrap();
    fs::write(&package, b"new package").unwrap();
    let destination = temp.path().join("backup");
    fs::create_dir(&destination).unwrap();
    fs::write(destination.join("stale-entry"), b"old snapshot").unwrap();

    assert!(backup_inner(&project, Some(destination.display().to_string())).is_err());
    assert_eq!(
        fs::read(destination.join("stale-entry")).unwrap(),
        b"old snapshot"
    );
    assert!(!destination.join("store.db").exists());
    assert!(!destination.join("vstore").exists());
}

#[test]
fn refuses_empty_project_without_creating_backup_destination() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let destination = temp.path().join("backup");

    assert!(backup_inner(&project, Some(destination.display().to_string())).is_err());
    assert!(!destination.exists());
}
