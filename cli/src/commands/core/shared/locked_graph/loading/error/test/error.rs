//! Diagnostic, source-chain and recovery tests for loading errors.
//! Test thông báo, chuỗi nguồn và hướng khắc phục lỗi loading.

use super::super::implementation::{LockedGraphLoadingError, LockedGraphRecovery};
use std::error::Error as StdError;

#[test]
fn lockfile_error_keeps_the_native_diagnostic_source_and_recovery() {
    let error = LockedGraphLoadingError::Lockfile(mgc_lockfile::LockfileError::ParseError(
        "fixture parse failure".to_string(),
    ));

    assert_eq!(error.to_string(), "Parse error: fixture parse failure");
    assert_eq!(
        StdError::source(&error)
            .expect("native lockfile error remains in source chain")
            .to_string(),
        "Parse error: fixture parse failure"
    );
    assert_eq!(error.recovery(), LockedGraphRecovery::RepairLockfile);
}

#[test]
fn invalid_utf8_keeps_the_existing_message_and_source() {
    let source = String::from_utf8(vec![0xFF]).expect_err("invalid fixture must fail UTF-8");
    let error = LockedGraphLoadingError::InvalidUtf8 { source };

    assert!(
        error
            .to_string()
            .starts_with("mgc.lock is not valid UTF-8: invalid utf-8 sequence")
    );
    assert!(StdError::source(&error).is_some());
    assert_eq!(error.recovery(), LockedGraphRecovery::RepairLockfile);
}

#[test]
fn project_owner_errors_keep_context_and_classify_configuration_recovery() {
    let error = LockedGraphLoadingError::ReadCoreMarker {
        source: anyhow::anyhow!("fixture marker failure"),
    };

    assert_eq!(
        error.to_string(),
        "cannot determine lock owner core: fixture marker failure"
    );
    assert!(StdError::source(&error).is_some());
    assert_eq!(
        error.recovery(),
        LockedGraphRecovery::CorrectProjectConfiguration
    );
}

#[test]
fn unknown_owner_keeps_the_refusal_message_without_inventing_a_source() {
    let error = LockedGraphLoadingError::UnknownCoreOwner {
        core: "unknown-core".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "refusing to read mgc.lock for unknown core owner 'unknown-core'"
    );
    assert!(StdError::source(&error).is_none());
    assert_eq!(
        error.recovery(),
        LockedGraphRecovery::CorrectProjectConfiguration
    );
}

#[test]
fn unsafe_lockfile_path_keeps_the_regular_file_refusal() {
    let error = LockedGraphLoadingError::UnsafeLockfilePath {
        path: "/tmp/project/mgc.lock".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "lockfile '/tmp/project/mgc.lock' must be a regular non-symlink file"
    );
    assert_eq!(error.recovery(), LockedGraphRecovery::RepairLockfile);
}

#[test]
fn only_transient_metadata_io_is_classified_for_retry() {
    let transient = LockedGraphLoadingError::MetadataRead(std::io::Error::from(
        std::io::ErrorKind::Interrupted,
    ));
    let permanent = LockedGraphLoadingError::MetadataRead(std::io::Error::from(
        std::io::ErrorKind::PermissionDenied,
    ));

    assert_eq!(transient.recovery(), LockedGraphRecovery::RetryRead);
    assert_eq!(permanent.recovery(), LockedGraphRecovery::CorrectFileAccess);
    let lockfile_io = LockedGraphLoadingError::Lockfile(mgc_lockfile::LockfileError::IoError(
        std::io::Error::from(std::io::ErrorKind::PermissionDenied),
    ));
    assert_eq!(
        lockfile_io.recovery(),
        LockedGraphRecovery::CorrectFileAccess
    );
}

#[test]
fn graph_conversion_errors_keep_the_native_typed_sources() {
    let legacy = LockedGraphLoadingError::LegacyGraphConversion(mgc_types::MgError::Other(
        "legacy graph fixture".to_string(),
    ));
    let v4 = LockedGraphLoadingError::V4GraphConversion(
        mgc_lockfile::LockfileError::UnusableForInstall("v4 graph fixture".to_string()),
    );

    assert_eq!(legacy.to_string(), "legacy graph fixture");
    assert_eq!(
        StdError::source(&legacy)
            .expect("legacy package error stays typed")
            .to_string(),
        "legacy graph fixture"
    );
    assert_eq!(
        v4.to_string(),
        "v4 lockfile cannot drive install losslessly: v4 graph fixture"
    );
    assert_eq!(
        StdError::source(&v4)
            .expect("v4 lockfile error stays typed")
            .to_string(),
        "v4 lockfile cannot drive install losslessly: v4 graph fixture"
    );
}
