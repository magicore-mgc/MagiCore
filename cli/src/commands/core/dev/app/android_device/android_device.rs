use anyhow::Result;
use std::path::Path;
use std::time::Duration;

/// Kiểm tra có AVD nào đang chạy qua adb không.
pub(in crate::commands::core::dev) fn android_emulator_running(root: &Path) -> Result<bool> {
    let report = run_android_device_command(root, &["devices"])?;
    Ok(adb_output_has_ready_device(&report.stdout_tail))
}

pub(in crate::commands::core::dev) fn run_android_device_command(
    root: &Path,
    args: &[&str],
) -> Result<mgc_exec::run::ExecReport> {
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        timeout: Some(Duration::from_secs(15)),
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::DeviceControl),
        ..Default::default()
    };
    mgc_exec::prelude::run("adb", &args, &opts).map_err(crate::error::android_device_command_failed)
}

/// Parse `adb devices` and accept only a serial whose state is exactly `device`.
/// Header text, offline, and unauthorized entries are not runnable targets.
/// Chỉ nhận serial có state chính xác là `device`; header/offline/unauthorized không chạy được.
pub(super) fn adb_output_has_ready_device(output: &str) -> bool {
    output.lines().any(|line| {
        let mut fields = line.split_whitespace();
        let Some(serial) = fields.next() else {
            return false;
        };
        let Some(state) = fields.next() else {
            return false;
        };
        serial != "List" && state == "device"
    })
}

#[cfg(test)]
#[path = "test/android_device.rs"]
mod tests;
