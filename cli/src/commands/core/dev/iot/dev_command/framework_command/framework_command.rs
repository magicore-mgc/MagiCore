//! Map IoT adapter identity to its existing monitor/build command.
//! Ánh xạ identity IoT adapter sang command monitor/build hiện tại.

use std::path::Path;

use super::super::error::IotDevError;

pub(in crate::commands::core::dev::iot::dev_command) fn iot_dev_command(
    root: &Path,
) -> anyhow::Result<(String, Vec<String>)> {
    let adapter = mgc_iot_adapter::adapter_for(root)
        .ok_or_else(|| crate::error::no_framework_detected("IoT", root))?;
    match adapter.framework() {
        "esp32-rust" => Ok(("espflash".to_string(), vec!["monitor".to_string()])),
        "platformio" => Ok(("pio".to_string(), vec!["run".to_string()])),
        "zephyr" => Ok(("west".to_string(), vec!["build".to_string()])),
        other => Err(IotDevError::UnsupportedFramework {
            framework: other.to_string(),
        }
        .into()),
    }
}
