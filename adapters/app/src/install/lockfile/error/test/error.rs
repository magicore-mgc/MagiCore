#![allow(clippy::unwrap_used)]

// Error mapping and source-chain contract tests.
// Test ánh xạ lỗi và hợp đồng source chain.
use super::super::*;

#[test]
fn local_lock_error_keeps_stable_context_and_message() {
    let error = AppLockfileError::message(
        "MGC_APP_LOCK_SCOPE",
        "validate ecosystem scope",
        "refusing to write mgc.lock without an owned ecosystem scope",
    );

    assert_eq!(error.code(), "MGC_APP_LOCK_SCOPE");
    assert_eq!(error.operation(), "validate ecosystem scope");
    assert!(!error.is_retryable());
    assert_eq!(
        mgc_types::MgError::from(error).to_string(),
        "refusing to write mgc.lock without an owned ecosystem scope"
    );
}

#[test]
fn caused_lock_error_retains_source_before_shared_mapping() {
    let error = AppLockfileError::caused(
        "MGC_APP_LOCK_PUBLISH",
        "publish canonical lock",
        "cannot atomically replace mgc.lock",
        std::io::Error::other("replace failed"),
    );

    assert_eq!(error.code(), "MGC_APP_LOCK_PUBLISH");
    assert_eq!(error.operation(), "publish canonical lock");
    assert!(std::error::Error::source(&error).is_some());
    assert!(!error.is_retryable());
    assert_eq!(
        mgc_types::MgError::from(error).to_string(),
        "cannot atomically replace mgc.lock"
    );
}
