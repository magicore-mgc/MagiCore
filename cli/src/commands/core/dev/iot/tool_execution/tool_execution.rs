use anyhow::{Result, bail};
use std::path::Path;

pub(super) fn execution_scope(cmd: &str) -> Result<mgc_exec::allowlist::ExecutionScope> {
    match cmd {
        "cargo" => Ok(mgc_exec::allowlist::ExecutionScope::BuildRunner),
        "espflash" => Ok(mgc_exec::allowlist::ExecutionScope::DeviceControl),
        _ => bail!("unsupported IoT tool"),
    }
}

pub(in crate::commands::core::dev) fn run_tool(
    root: &Path,
    cmd: &str,
    args: &[&str],
) -> Result<()> {
    let execution_scope = execution_scope(cmd)?;
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        clean_env: true,
        execution_scope: Some(execution_scope),
        ..Default::default()
    };
    let args = args.iter().map(|a| a.to_string()).collect::<Vec<_>>();
    mgc_exec::prelude::run_inherited(cmd, &args, &opts)?;
    Ok(())
}
