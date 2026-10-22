//! Tests for backup destination entry safety — kiểm tra an toàn entry đích.

use super::super::{regular_file_len, validate_backup_destination};
use std::fs;

#[cfg(unix)]
#[test]
fn destination_validation_rejects_a_symlink() {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("target");
    let link = directory.path().join("backup");
    fs::create_dir(&target).unwrap();
    symlink(&target, &link).unwrap();

    let error = validate_backup_destination(&link, None, None).unwrap_err();

    assert_eq!(
        error.to_string(),
        format!(
            "backup destination is a symlink or has an incompatible entry type: {}",
            link.display()
        )
    );
}

#[test]
fn destination_may_not_overlap_a_project_store_root() {
    let directory = tempfile::tempdir().unwrap();
    let store = directory.path().join("project/.magicore/cache/web");
    fs::create_dir_all(&store).unwrap();
    let store = fs::canonicalize(store).unwrap();
    let destination = store.join("backup");

    let error = validate_backup_destination(&destination, Some(&store), None).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("backup destination overlaps project Web-store data")
    );
}

#[test]
fn regular_file_measurement_handles_present_and_missing_files() {
    let directory = tempfile::tempdir().unwrap();
    let file = directory.path().join("store.db");
    fs::write(&file, b"db").unwrap();

    assert_eq!(regular_file_len(&file).unwrap(), 2);
    assert_eq!(
        regular_file_len(&directory.path().join("missing.db")).unwrap(),
        0
    );
}
