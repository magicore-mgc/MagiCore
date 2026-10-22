use super::super::reject_forbidden_pm_tool;
use crate::allowlist::ExecutionScope;

#[test]
fn deny_list_wins_before_scope_or_allowlist_checks() {
    let error = reject_forbidden_pm_tool("npm", "npm", ExecutionScope::Install).unwrap_err();
    assert!(error.to_string().contains("is forbidden in Install scope"));
    assert!(reject_forbidden_pm_tool("cargo", "cargo", ExecutionScope::Install).is_ok());
}
