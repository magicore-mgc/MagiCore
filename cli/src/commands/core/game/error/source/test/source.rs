use super::super::super::GameOptimizerError;
use std::error::Error as StdError;

#[test]
fn template_publish_errors_keep_the_filesystem_source() {
    let error = GameOptimizerError::template_publish_failed(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "publish denied",
    ));
    assert_eq!(
        error.to_string(),
        "optimizer template publish failed: publish denied"
    );
    assert_eq!(
        StdError::source(&error)
            .expect("publish source")
            .to_string(),
        "publish denied"
    );
}

#[test]
fn rollback_failures_report_both_failures_and_keep_rollback_source() {
    let original = anyhow::Error::msg("publish denied");
    let error = GameOptimizerError::manifest_rollback_failed(
        "template publish",
        &original,
        anyhow::Error::msg("refusing link-swapped project path"),
    );

    assert_eq!(
        error.to_string(),
        "optimizer template publish failed (publish denied); restoring Cargo.toml also failed: refusing link-swapped project path"
    );
    assert_eq!(
        StdError::source(&error)
            .expect("rollback source")
            .to_string(),
        "refusing link-swapped project path"
    );
}

#[test]
fn staging_cleanup_failure_reports_primary_failure_and_cleanup_source() {
    let original = anyhow::Error::msg("template materialization failed");
    let path = std::path::Path::new(".magicore/tmp-optimizer");
    let error = GameOptimizerError::staging_cleanup_failed(
        "materialize optimizer template",
        path,
        Some(&original),
        anyhow::Error::msg("cleanup permission denied"),
    );

    let rendered = error.to_string();
    assert!(rendered.contains("template materialization failed"));
    assert!(rendered.contains("cleanup permission denied"));
    assert!(rendered.contains(".magicore/tmp-optimizer"));
    assert_eq!(
        StdError::source(&error)
            .expect("cleanup source")
            .to_string(),
        "cleanup permission denied"
    );
}

#[test]
fn filesystem_errors_preserve_their_source() {
    let error = GameOptimizerError::native_manifest_inspect(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "access denied",
    ));
    assert_eq!(
        error.to_string(),
        "cannot inspect Bevy Cargo.toml: access denied"
    );
    assert_eq!(
        StdError::source(&error)
            .expect("filesystem source")
            .to_string(),
        "access denied"
    );
}

#[test]
fn non_causal_refusal_has_no_error_source() {
    let error = GameOptimizerError::project_not_detected(std::path::Path::new("project"));
    assert!(StdError::source(&error).is_none());
}
