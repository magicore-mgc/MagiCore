use anyhow::{Result, bail};

use crate::allowlist::ExecutionScope;

/// Return the canonical name of a tool that requires explicit device control.
/// Trả tên chuẩn của công cụ cần scope điều khiển thiết bị tường minh.
pub(crate) fn device_control_only_tool(normalized: &str) -> Option<&'static str> {
    match normalized {
        "adb" => Some("adb"),
        "xcrun" => Some("xcrun"),
        _ => None,
    }
}

/// Keep attached-device controls in the dedicated device-control scope.
/// Giới hạn lệnh điều khiển thiết bị trong scope DeviceControl chuyên biệt.
pub(in crate::allowlist::tool_check) fn restrict_device_tool_scope(
    normalized: &str,
    scope: ExecutionScope,
) -> Result<()> {
    if let Some(tool) = device_control_only_tool(normalized)
        && scope != ExecutionScope::DeviceControl
    {
        bail!("tool '{tool}' is only allowed in DeviceControl scope");
    }
    Ok(())
}
