//! Classify verified Windows descendants by command metadata and policy.
//! Phân loại descendant Windows đã xác minh theo command metadata và policy.

#[cfg(test)]
use super::super::{WindowsProcessScan, WindowsProcessSnapshot};
#[cfg(test)]
use crate::allowlist::ExecutionScope;
#[cfg(test)]
use crate::run::implementation::{ForbiddenProcess, forbidden_process_match_argv_with_scope};
#[cfg(test)]
use crate::run::implementation::{matched_invocation_shape_argv, process_basename};
#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
pub(super) fn scan_verified_descendant_commands(
    root_pid: u32,
    processes: &[WindowsProcessSnapshot],
    verified_current_pids: &std::collections::HashSet<u32>,
    exempt: &[&str],
) -> Result<WindowsProcessScan> {
    let mut missing_command_line = None;
    for process in processes
        .iter()
        .filter(|process| process.pid != root_pid && verified_current_pids.contains(&process.pid))
    {
        if process.command.is_empty() && missing_command_line.is_none() {
            missing_command_line = Some((process.pid, process.image_name.clone()));
        }
        if let Some((name, match_source, matched_token, matched_token_index)) =
            forbidden_process_match_argv_with_scope(
                &process.image_name,
                &process.command,
                exempt,
                &[],
                ExecutionScope::Install,
            )
        {
            return Ok(WindowsProcessScan::Forbidden(ForbiddenProcess {
                pid: process.pid,
                matched_operation: matched_invocation_shape_argv(
                    &name,
                    &process.command,
                    matched_token_index,
                ),
                name,
                image_name: Some(process_basename(&process.image_name)),
                match_source,
                matched_token,
                matched_token_index,
            }));
        }
    }

    Ok(match missing_command_line {
        Some((pid, image_name)) => WindowsProcessScan::MissingCommandLine { pid, image_name },
        None => WindowsProcessScan::Clean,
    })
}
