use super::super::json_file::write_new_private_file;
use crate::commands::package_runtime::registry_install_args;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn run_native_install(project_root: &Path, allow_scripts: bool) -> Result<()> {
    let args = registry_install_args(allow_scripts);
    let executable = std::env::current_exe()?;
    let options = mgc_exec::prelude::ExecOptions {
        cwd: Some(project_root.to_path_buf()),
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::Install),
        ..Default::default()
    };
    mgc_exec::prelude::run_project_binary_inherited(&executable, &args, &options)?;
    Ok(())
}
pub(in crate::commands::tool) fn require_web_lane() -> Result<()> {
    if crate::factory::available_cores()
        .iter()
        .all(|(core, _)| *core != "web")
    {
        return Err(crate::error::core_not_in_build("web"));
    }
    Ok(())
}
pub(in crate::commands::tool) fn install_with_project_credentials(
    project_root: &Path,
    caller_root: &Path,
    allow_scripts: bool,
) -> Result<()> {
    let source = caller_root.join(".npmrc");
    let destination = project_root.join(".npmrc");
    let copied =
        if let Some(content) = mgc_config::project::read_regular_project_text(&source, ".npmrc")? {
            write_new_private_file(&destination, content.as_bytes())?;
            true
        } else {
            false
        };
    let install = run_native_install(project_root, allow_scripts);
    let cleanup = if copied {
        fs::remove_file(&destination).context("remove temporary tool registry credentials")
    } else {
        Ok(())
    };
    match (install, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Err(error), Err(cleanup_error)) => {
            Err(error).context(format!("credential cleanup also failed: {cleanup_error:#}"))
        }
    }
}
