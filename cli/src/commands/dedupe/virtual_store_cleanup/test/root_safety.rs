//! Virtual-store root tests — no cleanup crosses a symlinked project boundary.
//! Test root virtual store — cleanup không vượt qua boundary project bằng symlink.
use super::super::{cleanup_unreferenced_vstore, vstore_root};
use super::symlink_support::symlink_directory;
use mgc_lockfile::Lockfile;
use std::fs;

#[cfg(any(unix, windows))]
#[test]
fn cleanup_refuses_symlinked_node_modules_and_preserves_external_store() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let external_node_modules = temp.path().join("external-node-modules");
    let external_orphan = external_node_modules.join(".magicore").join("orphan@1.0.0");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&external_orphan).unwrap();
    fs::write(external_orphan.join("payload.bin"), b"outside").unwrap();
    symlink_directory(&external_node_modules, &project.join("node_modules")).unwrap();

    let report = cleanup_unreferenced_vstore(&project, &Lockfile::new());

    assert_eq!(report.deleted_count(), 0);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 0);
    assert_eq!(report.failed_count(), 1);
    assert!(external_orphan.join("payload.bin").is_file());
}

#[cfg(any(unix, windows))]
#[test]
fn cleanup_refuses_symlinked_magicore_root_and_preserves_external_store() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let node_modules = project.join("node_modules");
    let external_store = temp.path().join("external-store");
    let external_orphan = external_store.join("orphan@1.0.0");
    fs::create_dir_all(&node_modules).unwrap();
    fs::create_dir_all(&external_orphan).unwrap();
    fs::write(external_orphan.join("payload.bin"), b"outside").unwrap();
    symlink_directory(&external_store, &vstore_root(&project)).unwrap();

    let report = cleanup_unreferenced_vstore(&project, &Lockfile::new());

    assert_eq!(report.deleted_count(), 0);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 0);
    assert_eq!(report.failed_count(), 1);
    assert!(external_orphan.join("payload.bin").is_file());
}

#[test]
fn cleanup_treats_missing_store_as_a_clean_noop() {
    let temp = tempfile::tempdir().unwrap();

    let report = cleanup_unreferenced_vstore(temp.path(), &Lockfile::new());

    assert_eq!(report.deleted_count(), 0);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 0);
    assert_eq!(report.failed_count(), 0);
}

#[test]
fn cleanup_rejects_non_directory_node_modules_root() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("node_modules"), b"not a directory").unwrap();

    let report = cleanup_unreferenced_vstore(temp.path(), &Lockfile::new());

    assert_eq!(report.deleted_count(), 0);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 0);
    assert_eq!(report.failed_count(), 1);
}
