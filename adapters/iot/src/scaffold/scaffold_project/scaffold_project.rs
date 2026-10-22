//! Validate the board and dispatch to its framework file generator.
//! Xác thực board và chuyển tới generator file của framework.

use super::super::framework::{
    scaffold_esp32, scaffold_platformio, scaffold_zephyr, validate_esp32_cargo_package_name,
};
use crate::framework::{IotFramework, board_target_for_framework, boards_for_framework};
use mgc_types::MgResult;
use std::path::Path;

/// Scaffold an IoT project only for a board registered to the selected framework.
/// Chỉ scaffold project IoT khi board được đăng ký cho framework đã chọn.
pub async fn scaffold_project(
    framework: IotFramework,
    project_name: &str,
    board: &str,
    target_dir: &Path,
) -> MgResult<()> {
    if board_target_for_framework(framework.as_str(), board).is_none() {
        let supported = boards_for_framework(framework.as_str())
            .iter()
            .map(|entry| entry.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(mgc_types::MgError::Unsupported {
            core: "iot",
            capability: "scaffold_board",
            guidance: format!(
                "board '{board}' is not registered for framework '{}'; supported boards: {supported}",
                framework.as_str()
            ),
        });
    }

    if matches!(framework, IotFramework::Esp32Rust) {
        validate_esp32_cargo_package_name(project_name)?;
    }

    std::fs::create_dir_all(target_dir)?;

    match framework {
        IotFramework::Esp32Rust => scaffold_esp32(project_name, board, target_dir).await,
        IotFramework::Platformio => scaffold_platformio(project_name, board, target_dir).await,
        IotFramework::Zephyr => scaffold_zephyr(project_name, board, target_dir).await,
    }
}
