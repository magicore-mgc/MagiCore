use super::super::GameOptimizerError;
use std::path::{Path, PathBuf};

#[test]
fn constructors_preserve_paths_and_original_error_context() {
    let root = Path::new("workspace/game");
    match GameOptimizerError::project_not_detected(root) {
        GameOptimizerError::ProjectNotDetected { root: actual } => {
            assert_eq!(actual, PathBuf::from(root));
        }
        other => panic!("unexpected error variant: {other:?}"),
    }

    let original = anyhow::Error::msg("publish denied").context("request failed");
    let expected = format!("{original:#}");
    match GameOptimizerError::manifest_rollback_failed(
        "template publish",
        &original,
        anyhow::Error::msg("restore denied"),
    ) {
        GameOptimizerError::ManifestRollbackFailed {
            operation,
            original_error,
            source,
        } => {
            assert_eq!(operation, "template publish");
            assert_eq!(original_error, expected);
            assert_eq!(source.to_string(), "restore denied");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }

    match GameOptimizerError::conflicting_dependency("./custom") {
        GameOptimizerError::ConflictingDependency { path } => assert_eq!(path, "./custom"),
        other => panic!("unexpected error variant: {other:?}"),
    }
}
