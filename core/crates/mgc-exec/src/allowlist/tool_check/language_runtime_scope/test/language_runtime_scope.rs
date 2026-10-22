use super::super::restrict_language_runtime_scope;
use crate::allowlist::ExecutionScope;

#[test]
fn language_runtime_requires_an_explicit_project_execution_scope() {
    assert!(restrict_language_runtime_scope("bash", "bash", ExecutionScope::BuildRunner).is_ok());
    let error =
        restrict_language_runtime_scope("bash", "bash", ExecutionScope::Install).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("requires a project execution scope")
    );
}
