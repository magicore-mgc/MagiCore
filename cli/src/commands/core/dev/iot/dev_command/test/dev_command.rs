//! Tests for IoT dev execution configuration.

use std::path::Path;

use super::super::execution_options;

#[test]
fn iot_dev_uses_device_control_scope_and_project_log() {
    let root = Path::new("/tmp/mgc-iot-project");
    let options = execution_options(root);
    let expected_log = root.join(".magicore").join("exec.log");

    assert_eq!(options.cwd.as_deref(), Some(root));
    assert_eq!(options.log_path.as_ref(), Some(&expected_log));
    assert!(options.clean_env);
    assert!(matches!(
        options.execution_scope,
        Some(mgc_exec::allowlist::ExecutionScope::DeviceControl)
    ));
}
