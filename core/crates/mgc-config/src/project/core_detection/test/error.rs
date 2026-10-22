//! Core-detection error-boundary tests.
//! Test biên lỗi nhận diện core.
use super::super::error::{CoreDetectionError, RecoveryAction};

#[test]
fn ambiguous_identity_recommends_explicit_core_selection() {
    let error = CoreDetectionError::AmbiguousProjectCore {
        project_root: std::path::PathBuf::from("sample"),
        signatures: vec!["game".to_string(), "web".to_string()],
        marker_file: ".mgc.core".to_string(),
    };

    assert_eq!(error.recovery_action(), RecoveryAction::SelectCore);
    assert!(error.to_string().contains("mgc init --signature <core>"));
}

#[test]
fn wrapped_source_error_remains_the_public_root_cause() {
    let source = anyhow::Error::msg("manifest parser source");
    let error = CoreDetectionError::from(source).into_anyhow();

    assert_eq!(error.root_cause().to_string(), "manifest parser source");
}
