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

/// `mgc dev --core lib` is a build-watch loop: a flag that belongs to a server
/// cannot be honoured, and ignoring it would leave the user believing a dev
/// server is running on that port.
/// (`mgc dev --core lib` là build-watch: cờ thuộc server không thể thực hiện, bỏ
/// qua sẽ khiến user tin có dev server trên port đó.)
pub fn dev_lib_flag_unsupported(flag: &str) -> Error {
    anyhow!(
        "'{flag}' does not apply to 'mgc dev --core lib': a library has no dev server, \
         so this command rebuilds on source change. Use `mgc build` for a one-shot build."
    )
}

/// The Hardware core has no dev loop by design: its packages are an optimizer
/// profile and a benchmark harness, both driven by other commands. Reporting
/// "not implemented yet" would promise a feature that will never exist.
/// (Core Hardware không có vòng dev theo thiết kế: package của nó là profile
/// optimizer và harness benchmark, đều do lệnh khác điều khiển. Báo "chưa
/// implement" là hứa một tính năng sẽ không bao giờ có.)
pub fn dev_hardware_not_applicable() -> Error {
    anyhow!(
        "'mgc dev' does not apply to the Hardware core: its packages are an optimizer profile \
         and a benchmark harness, not a program. Run `mgc optimizer` to write the profile or \
         `mgc bench` to measure."
    )
}

/// A runtime name reached the flag policy but is not one the policy models.
///
/// Fail closed: an unknown runtime has no flag profile, so every argument would
/// pass unchecked. Reaching this means a caller passed a runtime the policy does
/// not know — a bug to surface, not to absorb.
/// (Runtime lạ tới policy nhưng policy không mô hình hoá. Fail-closed: không có
/// profile flag nên mọi tham số sẽ đi qua không kiểm.)
pub fn dev_runtime_unknown_for_policy(runtime: &str) -> Error {
    anyhow!(
        "'{runtime}' has no launcher flag policy, so its arguments cannot be validated; \
         refusing to start it. Supported runtimes: node, tsx, bun, deno"
    )
}
