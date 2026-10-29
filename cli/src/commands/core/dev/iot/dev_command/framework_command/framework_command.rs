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
        // PAR-06: `mgc build` refuses `pio run`/`west build` because they may
        // resolve or install dependencies, yet `mgc dev` spawned exactly those
        // commands. One policy now covers both routes: only the read-only
        // monitors are allowed, and the build-shaped invocations stay refused
        // with the same guidance the build lane gives.
        // (PAR-06: `mgc build` từ chối `pio run`/`west build` vì chúng có thể
        // resolve/cài dependency, nhưng `mgc dev` lại spawn đúng hai lệnh đó.
        // Một chính sách giờ áp cho cả hai đường: chỉ cho monitor chỉ đọc, còn
        // lệnh dạng build vẫn bị từ chối với cùng hướng dẫn như lane build.)
        "platformio" => Err(IotDevError::UnsupportedFramework {
            framework:
                "platformio (build commands resolve dependencies; only monitoring is supported)"
                    .to_string(),
        }
        .into()),
        "zephyr" => Err(IotDevError::UnsupportedFramework {
            framework: "zephyr (build commands resolve dependencies; only monitoring is supported)"
                .to_string(),
        }
        .into()),
        other => Err(IotDevError::UnsupportedFramework {
            framework: other.to_string(),
        }
        .into()),
    }
}
