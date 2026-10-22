use super::super::{device_control_only_tool, restrict_device_tool_scope};
use crate::allowlist::ExecutionScope;

#[test]
fn adb_and_xcrun_are_device_scope_only() {
    assert!(restrict_device_tool_scope("adb", ExecutionScope::DeviceControl).is_ok());
    assert!(restrict_device_tool_scope("xcrun", ExecutionScope::DeviceControl).is_ok());
    assert!(restrict_device_tool_scope("adb", ExecutionScope::DevServer).is_err());
    assert!(restrict_device_tool_scope("xcrun", ExecutionScope::Install).is_err());
}

#[test]
fn device_control_tool_names_are_normalized_at_the_policy_boundary() {
    assert_eq!(device_control_only_tool("adb"), Some("adb"));
    assert_eq!(device_control_only_tool("xcrun"), Some("xcrun"));
    assert_eq!(device_control_only_tool("simctl"), None);
    assert_eq!(device_control_only_tool("node"), None);
}
