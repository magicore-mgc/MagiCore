use super::super::is_project_execution_scope;
use crate::allowlist::ExecutionScope;

#[test]
fn only_test_build_dev_and_script_scopes_allow_local_program_candidates() {
    for scope in [
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
    ] {
        assert!(is_project_execution_scope(scope));
    }
    assert!(!is_project_execution_scope(ExecutionScope::Install));
    assert!(!is_project_execution_scope(ExecutionScope::DeviceControl));
}
