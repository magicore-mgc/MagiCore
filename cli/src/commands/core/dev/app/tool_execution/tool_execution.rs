use anyhow::{Result, bail};
use std::path::Path;

fn app_tool_is_allowed(cmd: &str) -> bool {
    matches!(cmd, "xcodebuild" | "swift" | "flutter" | "gradle")
}

/// Run an explicitly selected app toolchain command with optional environment.
/// Chạy lệnh toolchain app được chọn tường minh với môi trường tùy chọn.
pub(crate) fn run_tool_with_env(
    root: &Path,
    cmd: &str,
    args: &[String],
    env: Option<Vec<(String, String)>>,
) -> Result<()> {
    if !app_tool_is_allowed(cmd) {
        bail!("unsupported app development tool");
    }
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env: env.unwrap_or_default(),
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::DevServer),
        ..Default::default()
    };
    mgc_exec::prelude::run_inherited(cmd, args, &opts)
        .map_err(|e| crate::error::app_tool_failed(cmd, &e))?;
    Ok(())
}

#[cfg(test)]
#[path = "test/tool_execution.rs"]
mod tests;
