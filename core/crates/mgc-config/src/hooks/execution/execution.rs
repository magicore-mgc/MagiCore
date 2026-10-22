//! Execute configured hooks through the shared process-policy boundary.
//! Chạy hook qua ranh giới policy process dùng chung.

use anyhow::{Result, bail};
use std::path::Path;

use super::super::command_parser::parse_hook_command;
use super::super::config::HooksConfig;
use super::super::dependency_policy::ensure_dependency_event_hooks_allowed;
use super::super::merge::load_merged;

/// Run all hooks for an event; any failed command fails the command.
/// Chạy hook cho event; một lệnh lỗi sẽ làm command thất bại.
pub fn run_hooks(project_root: &Path, event: &str) -> Result<()> {
    let cfg = load_merged(project_root)?;
    run_hooks_with_config(project_root, event, &cfg)
}

pub(in crate::hooks::owners) fn run_hooks_with_config(
    project_root: &Path,
    event: &str,
    cfg: &HooksConfig,
) -> Result<()> {
    ensure_dependency_event_hooks_allowed(cfg, event)?;

    let Some(cmds) = cfg.hooks.get(event) else {
        return Ok(());
    };
    for cmd in cmds {
        let argv = parse_hook_command(cmd)?;
        let Some((program, args)) = argv.split_first() else {
            continue;
        };
        let options = mgc_exec::run::ExecOptions {
            cwd: Some(project_root.to_path_buf()),
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            ..Default::default()
        };
        let report = mgc_exec::run::run_inherited(program, args, &options).map_err(|error| {
            anyhow::anyhow!("hook '{event}' was refused or failed to start: {error}")
        })?;
        if report.exit_code != 0 {
            bail!("hook '{event}' failed with exit code {}", report.exit_code);
        }
    }
    Ok(())
}
