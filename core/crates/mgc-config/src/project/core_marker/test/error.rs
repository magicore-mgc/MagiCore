//! Core-marker typed-error contract tests.
//! Test contract lỗi typed core marker.
use super::super::error::{CoreMarkerError, RecoveryAction};
use std::error::Error as StdError;
use std::path::PathBuf;

#[test]
fn domain_error_preserves_message_and_repair_classification() {
    let error = CoreMarkerError::UnknownCore {
        core: "monolith".to_string(),
        known: "web, ai".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "Unknown core 'monolith'. Expected one of: web, ai."
    );
    assert_eq!(
        error.recovery_action(),
        RecoveryAction::RepairProjectIdentity
    );
}

#[test]
fn concurrent_marker_change_is_classified_for_retry() {
    let error = CoreMarkerError::MarkerChangedDuringClaim {
        path: PathBuf::from("/tmp/.mgc.core"),
    };
    assert_eq!(error.recovery_action(), RecoveryAction::RetryOperation);
    assert!(error.to_string().contains("retry the command"));
}

#[test]
fn wrapped_source_remains_in_the_error_chain() {
    let error = CoreMarkerError::from(std::io::Error::other("disk unavailable"));
    assert_eq!(error.to_string(), "disk unavailable");
    assert!(StdError::source(&error).is_some());
    assert_eq!(error.recovery_action(), RecoveryAction::InspectSource);
}

#[test]
fn every_domain_variant_keeps_its_established_display_message() {
    use CoreMarkerError as Error;
    let cases = [
        (
            Error::ConfiguredCoreReassignment {
                existing: "web".into(),
                desired: "ai".into(),
            },
            "Project is already configured as core 'web'; refusing to replace it with 'ai'. Core reassignment is not supported by this version.",
        ),
        (
            Error::InvalidMarker {
                path: PathBuf::from(".mgc.core"),
                marker: "bad".into(),
                known: "web, ai".into(),
            },
            "'.mgc.core' has an invalid core marker 'bad'. Expected one of: web, ai. Repair the marker file manually before running core-aware commands.",
        ),
        (
            Error::MissingConfigEcosystem,
            "project config is missing its ecosystem",
        ),
        (
            Error::UnknownConfiguredCore {
                configured: "bad".into(),
                marker: "web".into(),
            },
            "project config has unknown core 'bad'; refusing to trust core marker 'web'",
        ),
        (
            Error::MarkerConfigConflict {
                marker: "ai".into(),
                configured: "web".into(),
            },
            "core marker 'ai' conflicts with mgc.toml core 'web'; core reassignment is not supported by this version",
        ),
        (
            Error::ExistingMarkerConflict {
                existing: "web".into(),
                desired: "ai".into(),
            },
            "Project is already marked as core 'web'; refusing to replace it with 'ai'. Core reassignment is not supported by this version.",
        ),
        (
            Error::ConcurrentMarkerConflict {
                existing: "web".into(),
                desired: "ai".into(),
            },
            "Project is already marked as core 'web'; refusing to replace it with 'ai'.",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn public_boundary_conversion_keeps_the_previous_anyhow_root_cause() {
    let expected = "Unknown core 'monolith'. Expected one of: web, ai.";
    let error = CoreMarkerError::UnknownCore {
        core: "monolith".to_string(),
        known: "web, ai".to_string(),
    }
    .into_anyhow();

    assert_eq!(error.to_string(), expected);
    assert_eq!(error.root_cause().to_string(), expected);
}
