use super::tool_execution::execution_scope;

#[test]
fn commands_receive_only_their_owned_execution_scope() {
    assert!(matches!(
        execution_scope("cargo"),
        Ok(mgc_exec::allowlist::ExecutionScope::BuildRunner)
    ));
    assert!(matches!(
        execution_scope("espflash"),
        Ok(mgc_exec::allowlist::ExecutionScope::DeviceControl)
    ));
    assert!(execution_scope("bash").is_err());
}
