//! Missing-marker error contract tests / Test contract lỗi marker thiếu.
//! Protects recovery guidance and diagnostic content.

use super::super::super::error::MissingMarkerAttestationError;

#[test]
fn missing_identity_error_keeps_marker_restore_guidance() {
    let error = MissingMarkerAttestationError::MissingIdentity {
        project: "/project".to_owned(),
        marker: ".mgc.core".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        concat!(
            "project core marker and attestation are missing for '/project' — restore ",
            "'.mgc.core' or run `mgc init --signature <core>` and `mgc trust anchor`"
        )
    );
}

#[test]
fn mismatch_error_keeps_trusted_identity_recovery_guidance() {
    let error = MissingMarkerAttestationError::Mismatch {
        project: "/project".to_owned(),
        reason: "identity changed".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        concat!(
            "core identity attestation mismatch for '/project': identity changed — restore the ",
            "trusted project identity before continuing"
        )
    );
}

#[test]
fn verification_error_retains_its_source() {
    let error = MissingMarkerAttestationError::CannotVerify {
        source: anyhow::Error::msg("underlying failure"),
    }
    .into_anyhow();

    assert_eq!(
        error.to_string(),
        "cannot verify core identity attestation (underlying failure)"
    );
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "underlying failure")
    );
}
