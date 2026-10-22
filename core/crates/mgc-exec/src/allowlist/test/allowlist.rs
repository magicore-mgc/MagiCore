use crate::allowlist::{ExecutionScope, check_tool};

#[test]
fn compatibility_wrapper_keeps_default_install_policy() {
    assert!(check_tool("cargo").is_ok());
    assert!(check_tool("npm").is_err());
    assert!(!ExecutionScope::Install.allows_pm_tools());
    assert!(!ExecutionScope::FrameworkInitializer.allows_pm_tools());
}
