use crate::allowlist::ExecutionScope;

#[test]
fn install_scope_constraints_preserve_current_policy() {
    let scope = ExecutionScope::Install.constraints();
    assert!(!scope.cwd_locked);
    assert!(!scope.no_network);
    assert!(!scope.no_arbitrary_args);
    assert!(scope.audit_log_required);
    assert!(scope.shell_injection_check);
}

#[test]
fn project_execution_scope_constraints_lock_cwd_and_audit() {
    for scope in [
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
        ExecutionScope::FrameworkInitializer,
        ExecutionScope::DeviceControl,
    ] {
        let constraints = scope.constraints();
        assert!(constraints.cwd_locked);
        assert!(constraints.no_arbitrary_args);
        assert!(constraints.audit_log_required);
        assert!(constraints.shell_injection_check);
    }
}
