//! Verifies remediation status controls the command result.
//! Kiểm tra status remediation quyết định kết quả command.

use crate::commands::doctor::storage::{RemediationOutcome, RemediationStatus, StorageError};

use super::super::ensure_remediation_completed;

#[test]
fn verified_or_already_writable_store_returns_success() {
    for status in [
        RemediationStatus::Created,
        RemediationStatus::AlreadyWritable,
    ] {
        let outcome = RemediationOutcome {
            status,
            message: "verified".to_string(),
            error: None,
        };
        assert!(ensure_remediation_completed(outcome).is_ok());
    }
}

#[test]
fn skipped_repair_returns_failure() {
    let outcome = RemediationOutcome {
        status: RemediationStatus::Skipped,
        message: "existing path was left unchanged".to_string(),
        error: None,
    };

    assert!(ensure_remediation_completed(outcome).is_err());
}

#[test]
fn failed_repair_preserves_the_storage_error_type() {
    let outcome = RemediationOutcome {
        status: RemediationStatus::Failed,
        message: "store creation failed".to_string(),
        error: Some(StorageError::CreateStore {
            path: "/protected/store".into(),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        }),
    };

    let error = ensure_remediation_completed(outcome).unwrap_err();

    assert!(error.downcast_ref::<StorageError>().is_some());
}
