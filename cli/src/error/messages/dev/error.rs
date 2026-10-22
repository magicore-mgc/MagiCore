//! Development runtime and framework diagnostics.
//! Lỗi dev runtime và framework.

use anyhow::Error;
use anyhow::anyhow;

pub fn path_not_utf8() -> Error {
    anyhow!("project path is not valid UTF-8")
}

pub fn unreal_editor_dev(uproject: &str) -> Error {
    anyhow!(
        "unreal dev runs in Unreal Editor (engine binary is named by version, no standard PATH) — open {uproject} in the editor, or install UnrealBuildTool and run the build directly. Run `mgc build` + the editor to run."
    )
}

pub fn dev_no_command_for_engine(engine: &str) -> Error {
    anyhow!("'mgc dev' has no command for engine '{engine}' — use the engine editor to run it")
}

pub fn godot_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "godot failed: {e} — install the Godot editor first (https://godotengine.org) and ensure `godot` is in PATH"
    )
}

pub fn multi_dev_flutter_only() -> Error {
    anyhow!("multi dev P1 targets flutter/ entry — run `mgc build` for other platforms")
}

/// package.json scripts must remain a map of string commands.
/// Bảng scripts trong package.json phải giữ cấu trúc map lệnh dạng chuỗi.
/// dev chưa implement cho core này
pub fn dev_core_not_implemented(core: &str) -> Error {
    anyhow!("'mgc dev' Engine is not implemented for the '{core}' core yet")
}

/// dev chưa implement cho iot framework này
pub fn dev_iot_framework_not_implemented(fw: &str) -> Error {
    anyhow!("'mgc dev' for '{fw}' iot framework is not implemented yet")
}

/// dev chưa implement cho cloud type này
pub fn dev_cloud_not_implemented(cloud: &str) -> Error {
    anyhow!(
        "'mgc dev' for '{cloud}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}
