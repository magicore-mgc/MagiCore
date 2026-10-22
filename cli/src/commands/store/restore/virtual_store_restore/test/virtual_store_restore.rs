//! Virtual-store restore cases.
//! Ca khôi phục virtual store.

use super::super::super::error::VirtualStoreRestoreError;
use super::super::restore_virtual_store;
use std::fs;
use std::path::Path;

#[test]
fn replaces_existing_virtual_store_contents() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("backup/vstore");
    let destination = temp.path().join("project/node_modules/.magicore");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&destination).unwrap();
    fs::write(source.join("new.bin"), b"new").unwrap();
    fs::write(destination.join("old.bin"), b"old").unwrap();

    restore_virtual_store(&source, &destination).unwrap();

    assert!(destination.join("new.bin").is_file());
    assert!(!destination.join("old.bin").exists());
}

#[test]
fn refuses_missing_backup_tree_before_clearing_project_contents() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("backup/missing-vstore");
    let destination = temp.path().join("project/node_modules/.magicore");
    fs::create_dir_all(&destination).unwrap();
    fs::write(destination.join("keep.bin"), b"keep").unwrap();

    assert!(restore_virtual_store(&source, &destination).is_err());
    assert_eq!(fs::read(destination.join("keep.bin")).unwrap(), b"keep");
}

#[cfg(any(unix, windows))]
#[test]
fn restores_relative_internal_directory_link_after_target_contents() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("backup/vstore");
    let destination = temp.path().join("project/node_modules/.magicore");
    let target = source.join("target");
    let link = source.join("links/package");
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(link.parent().unwrap()).unwrap();
    fs::write(target.join("package.json"), b"target package").unwrap();
    create_directory_symlink(Path::new("../target"), &link).unwrap();

    restore_virtual_store(&source, &destination).unwrap();

    let restored_link = destination.join("links/package");
    assert_eq!(
        fs::read(restored_link.join("package.json")).unwrap(),
        b"target package"
    );
    assert_eq!(
        fs::canonicalize(&restored_link).unwrap(),
        fs::canonicalize(destination.join("target")).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn refuses_symlinked_virtual_store_root_without_clearing_its_target() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("backup/vstore");
    let target = temp.path().join("outside");
    let destination_parent = temp.path().join("project/node_modules");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(&destination_parent).unwrap();
    fs::write(target.join("keep.bin"), b"keep").unwrap();
    symlink(&target, destination_parent.join(".magicore")).unwrap();

    assert!(matches!(
        restore_virtual_store(&source, &destination_parent.join(".magicore")),
        Err(VirtualStoreRestoreError::Clear(_))
    ));
    assert_eq!(fs::read(target.join("keep.bin")).unwrap(), b"keep");
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
