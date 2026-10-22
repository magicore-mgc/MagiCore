use anyhow::Result;
use std::path::Path;

pub(crate) fn run_allowlisted_tool(root: &Path, program: &str, args: &[&str]) -> Result<()> {
    run_allowlisted_tool_with_env(root, program, args, None)
}

pub(crate) fn run_allowlisted_tool_with_env(
    root: &Path,
    program: &str,
    args: &[&str],
    env: Option<Vec<(String, String)>>,
) -> Result<()> {
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env: env.unwrap_or_default(),
        clean_env: false, // Preserve env when custom env provided
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
        ..Default::default()
    };
    let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
    mgc_exec::prelude::run(program, &args, &opts)?;
    Ok(())
}
