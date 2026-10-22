//! Verifies repair changes only a missing store path.
//! Kiểm tra sửa chữa chỉ thay đổi path store còn thiếu.

use std::fs;

use super::super::{RemediationStatus, initialize_missing, initialize_missing_at};

#[test]
fn creates_and_verifies_only_a_missing_store() {
    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("nested").join("store");

    let result = initialize_missing_at(store.clone());

    assert!(matches!(result.outcome.status, RemediationStatus::Created));
    assert!(store.is_dir());
    assert!(result.outcome.message.contains("verified"));
    assert!(result.store.writable);
    assert_eq!(fs::read_dir(&store).unwrap().count(), 0);
}

#[test]
fn leaves_a_non_directory_path_unchanged() {
    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("store-file");
    fs::write(&store, b"preserve").unwrap();

    let result = initialize_missing_at(store.clone());

    assert!(matches!(result.outcome.status, RemediationStatus::Skipped));
    assert_eq!(fs::read(&store).unwrap(), b"preserve");
    assert!(result.outcome.message.contains("manually"));
}

#[cfg(unix)]
#[test]
fn leaves_a_broken_symlink_unchanged_instead_of_creating_its_target() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("target");
    let store = root.path().join("store-link");
    symlink(&target, &store).unwrap();

    let result = initialize_missing_at(store.clone());

    assert!(matches!(result.outcome.status, RemediationStatus::Skipped));
    assert!(
        fs::symlink_metadata(&store)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(!target.exists());
}

#[cfg(unix)]
#[test]
fn does_not_follow_a_symlink_inserted_after_the_missing_snapshot() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("store");
    let target = root.path().join("target");
    let initial = super::super::super::inspect_at(store.clone());
    symlink(&target, &store).unwrap();

    let result = initialize_missing(initial);

    assert!(matches!(result.outcome.status, RemediationStatus::Skipped));
    assert!(
        fs::symlink_metadata(&store)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(!target.exists());
}

#[cfg(unix)]
#[test]
fn verifies_a_writable_symlink_inserted_after_snapshot_without_leaving_probe_files() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("store");
    let target = root.path().join("target");
    fs::create_dir(&target).unwrap();
    let initial = super::super::super::inspect_at(store.clone());
    symlink(&target, &store).unwrap();

    let result = initialize_missing(initial);

    assert!(matches!(
        result.outcome.status,
        RemediationStatus::AlreadyWritable
    ));
    assert!(
        fs::symlink_metadata(&store)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read_dir(&target).unwrap().count(), 0);
}

#[test]
fn skips_repair_when_a_non_directory_ancestor_exists() {
    let root = tempfile::tempdir().unwrap();
    let parent_file = root.path().join("parent-file");
    fs::write(&parent_file, b"preserve").unwrap();
    let store = parent_file.join("store");

    let result = initialize_missing_at(store);

    assert!(matches!(result.outcome.status, RemediationStatus::Skipped));
    assert!(result.outcome.message.contains("not a directory"));
    assert!(result.outcome.error.is_none());
    assert_eq!(fs::read(parent_file).unwrap(), b"preserve");
}
