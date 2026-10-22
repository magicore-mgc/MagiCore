use anyhow::Result;
use std::path::Path;
use std::time::Duration;

/// Find an Xcode workspace first, then a project, without recursive traversal.
/// Tìm workspace Xcode trước, sau đó project, không duyệt đệ quy.
pub(in crate::commands::core::dev) fn find_xcode_project(root: &Path) -> Option<String> {
    let mut workspace: Option<String> = None;
    let mut project: Option<String> = None;
    for entry in std::fs::read_dir(root).ok()?.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".xcworkspace") && workspace.is_none() {
            workspace = Some(name);
        } else if name.ends_with(".xcodeproj") && project.is_none() {
            project = Some(name);
        }
    }
    workspace.or(project)
}

/// Run an Xcode simulator/device probe through the audited executor.
/// Detection call sites have no project root, so they log nothing
/// project-local; allowlist + DeviceControl scope + redaction still apply.
/// Chạy probe simulator/thiết bị Xcode qua executor có audit.
/// Điểm detect không có project root nên không ghi log project-local;
/// allowlist + scope DeviceControl + redact vẫn áp dụng.
pub(super) fn run_xcrun(
    args: &[&str],
    project_log_dir: Option<&Path>,
) -> Result<mgc_exec::run::ExecReport> {
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    let opts = mgc_exec::prelude::ExecOptions {
        log_path: project_log_dir.map(|dir| dir.join(".magicore").join("exec.log")),
        timeout: Some(Duration::from_secs(15)),
        capture_full_stdout: true,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::DeviceControl),
        ..Default::default()
    };
    mgc_exec::prelude::run("xcrun", &args, &opts)
        .map_err(|e| crate::error::app_tool_failed("xcrun", &e))
}

/// Kiểm tra Xcode CLI tools có sẵn (xcrun tồn tại và simctl hoạt động).
pub(in crate::commands::core::dev) fn xcode_available() -> bool {
    xcode_available_in(None)
}

/// Kiểm tra Xcode với audit log project khi có root.
/// Check Xcode with project audit log when root is known.
pub(in crate::commands::core::dev) fn xcode_available_in(project_root: Option<&Path>) -> bool {
    run_xcrun(&["simctl", "list", "devices", "--json"], project_root)
        .map(|report| report.exit_code == 0)
        .unwrap_or(false)
}

/// Lấy UDID của simulator iOS đang booted (ưu tiên) hoặc simulator available đầu tiên.
/// Trả về `None` nếu không có simulator nào.
pub fn find_ios_simulator() -> Option<String> {
    find_ios_simulator_in(None)
}

/// Lấy UDID simulator với audit log project khi có root.
/// Find simulator UDID with project audit log when root is known.
pub fn find_ios_simulator_in(project_root: Option<&Path>) -> Option<String> {
    let report = run_xcrun(&["simctl", "list", "devices", "--json"], project_root).ok()?;
    let json: serde_json::Value = serde_json::from_str(&report.stdout_full).ok()?;
    let devices = json.get("devices")?.as_object()?;

    let mut booted_udid: Option<String> = None;
    let mut any_available_udid: Option<String> = None;

    // Duyệt tất cả runtime (iOS 17/18...) tìm device sẵn
    for (_runtime, devs) in devices {
        let Some(arr) = devs.as_array() else { continue };
        for dev in arr {
            let name = dev.get("name").and_then(|v| v.as_str()).unwrap_or("");
            // Chỉ xét iPhone, bỏ Watch/TV/Vision
            if !name.to_lowercase().contains("iphone") {
                continue;
            }
            let avail = dev
                .get("isAvailable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !avail {
                continue;
            }
            let udid = dev.get("udid").and_then(|v| v.as_str())?;
            let state = dev.get("state").and_then(|v| v.as_str()).unwrap_or("");
            if state == "Booted" {
                booted_udid = Some(udid.to_string());
                break;
            }
            if any_available_udid.is_none() {
                any_available_udid = Some(udid.to_string());
            }
        }
        if booted_udid.is_some() {
            break;
        }
    }
    booted_udid.or(any_available_udid)
}

/// Boot simulator nếu chưa chạy (non-blocking — trả về khi boot xong)
/// với audit log project khi có root. Best-effort:
/// boot báo lỗi khi simulator đã booted nên mọi lỗi đều bỏ qua
/// (callers cũng đã dùng `.ok()`).
/// Boot simulator with project audit log when root is known.
pub(in crate::commands::core::dev) fn boot_simulator_in(
    udid: &str,
    project_root: Option<&Path>,
) -> Result<()> {
    // boot trả về lỗi nếu đã booted — bỏ qua
    let _ = run_xcrun(&["simctl", "boot", udid], project_root);
    Ok(())
}

#[cfg(test)]
#[path = "test/ios_simulator.rs"]
mod tests;
