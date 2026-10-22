//! Trust-pending typed error contracts — contract lỗi typed trust-pending.

use super::*;
use std::error::Error;
use std::io;

#[test]
fn pending_journal_keeps_original_message_and_typed_category() {
    let error = TrustPendingError::from(anyhow::anyhow!("journal recovery required"));

    assert_eq!(error.to_string(), "journal recovery required");
    assert!(matches!(error, TrustPendingError::PendingJournal(_)));
}

#[test]
fn scan_error_keeps_display_source_and_typed_category() {
    let error = TrustPendingError::scan_failed(
        Path::new("node_modules/pkg/package.json"),
        io::Error::other("read denied"),
    );

    assert_eq!(
        error.to_string(),
        "failed to inspect lifecycle-script package data at 'node_modules/pkg/package.json': read denied"
    );
    assert!(error.source().is_some());
    assert!(matches!(error, TrustPendingError::ScanFailed { .. }));
}

#[test]
fn lock_error_keeps_stable_message_and_typed_category() {
    let error =
        TrustPendingError::lock_failed(Path::new("project"), io::Error::other("writer busy"));

    assert_eq!(
        error.to_string(),
        "cannot scan trust-pending packages in 'project' while project state is changing or unrecovered: writer busy"
    );
    assert!(error.source().is_some());
    assert!(matches!(error, TrustPendingError::LockFailed { .. }));
}

#[test]
fn policy_error_keeps_source_and_typed_category() {
    let error = TrustPendingError::scripts_policy_failed(io::Error::other("invalid scripts table"));

    assert_eq!(
        error.to_string(),
        "failed to load [scripts] trust policy: invalid scripts table"
    );
    assert!(error.source().is_some());
    assert!(matches!(
        error,
        TrustPendingError::ScriptsPolicyInvalid { .. }
    ));
}

#[test]
fn invalid_manifest_has_specific_message_and_typed_category() {
    let error = TrustPendingError::manifest_invalid(Path::new("package.json"), "bad shape");

    assert_eq!(
        error.to_string(),
        "invalid lifecycle-script package manifest 'package.json': bad shape"
    );
    assert!(matches!(error, TrustPendingError::ManifestInvalid { .. }));
}

#[test]
fn oversized_manifest_has_specific_message_and_typed_category() {
    let error = TrustPendingError::manifest_too_large(Path::new("package.json"), 1024);

    assert_eq!(
        error.to_string(),
        "lifecycle-script package manifest 'package.json' exceeds the 1024-byte inspection limit"
    );
    assert!(matches!(error, TrustPendingError::ManifestTooLarge { .. }));
}
