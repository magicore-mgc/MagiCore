//! Existing tests for AI diagnostic and causal-source contracts.
//! Test hiện có cho diagnostic và source chain lỗi AI.

use super::super::AiCoreError;
use std::error::Error as StdError;

#[test]
fn ai_domain_errors_keep_the_existing_diagnostic_text() {
    let project = AiCoreError::project_not_detected();
    assert_eq!(
        project.to_string(),
        "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework)."
    );
    assert!(StdError::source(&project).is_none());

    let temp = tempfile::tempdir().expect("tempdir");
    let framework = AiCoreError::framework_not_detected(temp.path());
    assert_eq!(
        framework.to_string(),
        format!("No ai framework detected in {}", temp.path().display())
    );
    assert!(StdError::source(&framework).is_none());

    let unavailable = AiCoreError::native_dependency_lifecycle_unavailable("install");
    assert_eq!(
        unavailable.to_string(),
        "`ai` install for `python` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
    );
    assert!(StdError::source(&unavailable).is_none());
}

#[test]
fn python_runtime_error_keeps_the_original_execution_cause() {
    let cause = anyhow::Error::new(std::io::Error::other("python process failed"));
    let error = AiCoreError::python_runtime_failed("python3", cause);

    assert_eq!(
        error.to_string(),
        "Python launcher 'python3' failed: python process failed"
    );
    assert_eq!(
        StdError::source(&error)
            .expect("runtime source")
            .to_string(),
        "python process failed"
    );
}
