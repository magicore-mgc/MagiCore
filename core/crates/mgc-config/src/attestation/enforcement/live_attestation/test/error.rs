//! Live-error contract tests / Test contract lỗi live.
//! Verifies stable display messages and error source retention.

use super::super::super::error::LiveAttestationError;

#[test]
fn missing_error_preserves_the_require_diagnostic() {
    let error = LiveAttestationError::Missing {
        project: "/project".to_owned(),
        core: "web".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        concat!(
            "no core attestation for '/project' (core 'web') — run `mgc trust anchor` to ",
            "attest this project (or set MGC_TRUST_ANCHOR=warn to downgrade)"
        )
    );
}

#[test]
fn mismatch_error_preserves_reason_and_recovery_command() {
    let error = LiveAttestationError::Mismatch {
        project: "/project".to_owned(),
        core: "web".to_owned(),
        reason: "identity changed".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        concat!(
            "core attestation mismatch for '/project' (core 'web'): identity changed — run ",
            "`mgc trust anchor --re-attest` after intentional changes"
        )
    );
}

#[test]
fn verification_error_preserves_the_public_diagnostic() {
    let error = LiveAttestationError::CannotVerify {
        source: anyhow::Error::msg("corrupt state"),
    }
    .into_anyhow();

    assert_eq!(
        error.to_string(),
        "cannot verify core attestation (corrupt state)"
    );
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "corrupt state")
    );
}

#[test]
fn global_identity_error_retains_its_source() {
    let error = LiveAttestationError::GlobalIdentitiesUnavailable {
        source: anyhow::Error::msg("underlying failure"),
    }
    .into_anyhow();

    assert_eq!(
        error.to_string(),
        "cannot access global identities dir (underlying failure) — core attestation cannot be checked"
    );
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "underlying failure")
    );
}
