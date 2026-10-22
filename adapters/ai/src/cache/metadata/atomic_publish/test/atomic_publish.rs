//! Atomic publication cases — kiểm tra safety và concurrent replacement.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::atomic_publish::{publish_metadata, publish_metadata_with_replace};
use crate::cache::metadata::MetadataError;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn creates_sidecar_with_complete_bytes() {
    let temp = TempDir::new().unwrap();
    let sidecar = temp.path().join("model.meta.json");

    publish_metadata(&sidecar, b"complete-json").unwrap();

    assert_eq!(std::fs::read(&sidecar).unwrap(), b"complete-json");
}

#[test]
fn concurrent_saves_leave_one_complete_payload() {
    let temp = TempDir::new().unwrap();
    let sidecar = temp.path().join("model.meta.json");
    let first = b"first-complete-json".to_vec();
    let second = b"second-complete-json".to_vec();
    std::thread::scope(|scope| {
        let first_path = &sidecar;
        let first_bytes = &first;
        let second_path = &sidecar;
        let second_bytes = &second;
        let left = scope.spawn(move || publish_metadata(first_path, first_bytes));
        let right = scope.spawn(move || publish_metadata(second_path, second_bytes));
        left.join().unwrap().unwrap();
        right.join().unwrap().unwrap();
    });

    let contents = std::fs::read(&sidecar).unwrap();
    assert!(contents == first || contents == second);
}

#[cfg(unix)]
#[test]
fn new_sidecar_permissions_are_owner_only() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempDir::new().unwrap();
    let sidecar = temp.path().join("model.meta.json");
    publish_metadata(&sidecar, b"json").unwrap();

    assert_eq!(
        std::fs::metadata(sidecar).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[cfg(unix)]
#[test]
fn rejects_symlink_destination_without_modifying_target_or_leaving_staging() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let destination = temp.path().join("model.meta.json");
    let target = temp.path().join("outside.json");
    std::fs::write(&target, b"outside").unwrap();
    symlink(&target, &destination).unwrap();

    assert!(matches!(
        publish_metadata(&destination, b"replacement"),
        Err(MetadataError::SymlinkSidecar)
    ));
    assert_eq!(std::fs::read(&target).unwrap(), b"outside");
    assert!(std::fs::read_dir(temp.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")
    }));
}

#[test]
fn rejects_directory_destination_without_creating_staging() {
    let temp = TempDir::new().unwrap();
    let destination = temp.path().join("model.meta.json");
    std::fs::create_dir(&destination).unwrap();

    assert!(matches!(
        publish_metadata(&destination, b"replacement"),
        Err(MetadataError::NonRegularSidecar)
    ));
    assert_eq!(std::fs::read_dir(&destination).unwrap().count(), 0);
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
}

#[test]
fn removes_owned_staging_when_atomic_replace_fails() {
    let temp = TempDir::new().unwrap();
    let destination = temp.path().join("model.meta.json");
    std::fs::write(&destination, b"original").unwrap();

    let error = publish_metadata_with_replace(&destination, b"replacement", |_, _| {
        Err(std::io::Error::other("injected replace failure"))
    })
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("atomically replace metadata sidecar")
    );
    assert_eq!(std::fs::read(&destination).unwrap(), b"original");
    assert!(std::fs::read_dir(temp.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".mgc-ai-metadata-")
    }));
}

#[cfg(unix)]
#[test]
fn preserves_existing_regular_sidecar_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempDir::new().unwrap();
    let sidecar = temp.path().join("model.meta.json");
    std::fs::write(&sidecar, b"old").unwrap();
    std::fs::set_permissions(&sidecar, std::fs::Permissions::from_mode(0o640)).unwrap();

    publish_metadata(&sidecar, b"new").unwrap();

    assert_eq!(
        std::fs::metadata(sidecar).unwrap().permissions().mode() & 0o777,
        0o640
    );
}

#[test]
fn invalid_path_without_filename_is_rejected() {
    let error = publish_metadata(Path::new(""), b"json").unwrap_err();

    assert!(matches!(error, MetadataError::InvalidSidecarPath));
}
