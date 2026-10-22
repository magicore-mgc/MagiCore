//! Checks stable store error wording and low-disk issue ordering.
//! Kiểm tra text lỗi store và thứ tự issue disk thấp ổn định.

use super::super::diagnostics;
use crate::commands::doctor::storage::{StorageError, StoreSnapshot, StoreStatus};

#[test]
fn missing_store_precedes_low_disk_warning_and_suggests_bounded_repair() {
    let store = StoreSnapshot {
        directory: "/configured/mgc/store".into(),
        writable: false,
        free_bytes: 1,
        volume_path: Some("/configured/mgc".into()),
        status: StoreStatus::Missing,
        free_space_error: None,
    };
    let (issues, actions) = diagnostics(&store);

    assert_eq!(issues[0].code, "ERR_STORE_MISSING");
    assert_eq!(issues[0].severity, "CRITICAL");
    assert!(issues[0].message.contains("/configured/mgc/store"));
    assert_eq!(
        issues[0].fix_command.as_deref(),
        Some("mgc doctor check --fix")
    );
    assert_eq!(issues[1].code, "WARN_LOW_DISK_SPACE");
    assert_eq!(actions.len(), 2);
}

#[test]
fn existing_unwritable_store_does_not_offer_recursive_chmod() {
    let store = StoreSnapshot {
        directory: "/configured/mgc/store".into(),
        writable: false,
        free_bytes: 1024 * 1024 * 1024,
        volume_path: Some("/configured/mgc".into()),
        status: StoreStatus::NotWritable(StorageError::CreateWriteProbe {
            directory: "/configured/mgc/store".into(),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        }),
        free_space_error: None,
    };
    let (issues, _) = diagnostics(&store);

    assert_eq!(issues[0].code, "ERR_STORE_NOT_WRITABLE");
    assert!(issues[0].fix_command.is_none());
    assert!(!issues[0].message.contains("chmod"));
}

#[test]
fn non_directory_store_has_no_automatic_repair_command() {
    let store = StoreSnapshot {
        directory: "/configured/mgc/store".into(),
        writable: false,
        free_bytes: 1024 * 1024 * 1024,
        volume_path: Some("/configured/mgc".into()),
        status: StoreStatus::NotDirectory,
        free_space_error: None,
    };
    let (issues, _) = diagnostics(&store);

    assert_eq!(issues[0].code, "ERR_STORE_NOT_DIRECTORY");
    assert!(issues[0].fix_command.is_none());
}

#[test]
fn broken_symlink_has_a_specific_manual_repair_diagnostic() {
    let store = StoreSnapshot {
        directory: "/configured/mgc/store-link".into(),
        writable: false,
        free_bytes: 1024 * 1024 * 1024,
        volume_path: Some("/configured/mgc".into()),
        status: StoreStatus::BrokenSymlink {
            path: "/configured/mgc/store-link".into(),
        },
        free_space_error: None,
    };
    let (issues, actions) = diagnostics(&store);

    assert_eq!(issues[0].code, "ERR_STORE_BROKEN_SYMLINK");
    assert!(issues[0].fix_command.is_none());
    assert!(actions[0].contains("manually"));
}

#[test]
fn unknown_disk_space_is_explicitly_reported() {
    let store = StoreSnapshot {
        directory: "/configured/mgc/store".into(),
        writable: true,
        free_bytes: 0,
        volume_path: Some("/configured/mgc".into()),
        status: StoreStatus::Writable,
        free_space_error: Some(StorageError::AvailableSpace {
            path: "/configured/mgc".into(),
            source: std::io::Error::other("not available"),
        }),
    };
    let (issues, _) = diagnostics(&store);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].code, "WARN_STORE_DISK_UNKNOWN");
}
