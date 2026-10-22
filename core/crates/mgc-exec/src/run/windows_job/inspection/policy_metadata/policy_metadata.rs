//! Policy classification for metadata from confirmed Windows Job members.
//! Phân loại policy từ metadata của member Windows Job đã xác nhận.

#[cfg(any(windows, test))]
use super::super::super::npm_probe::windows_npm_probe_authentication;
#[cfg(any(windows, test))]
use super::super::WindowsProcessSnapshot;
#[cfg(any(windows, test))]
use super::super::{WindowsJobInspection, WindowsProcessPolicyMetadata};
#[cfg(any(windows, test))]
use crate::allowlist::ExecutionScope;
#[cfg(any(windows, test))]
use crate::run::implementation::{
    CompatAuditContext, ForbiddenProcess, ShadowPath, audit_compat_process,
    forbidden_process_match_argv_with_scope, is_npm_version_shell_probe,
    matched_invocation_shape_argv, process_basename,
};
#[cfg(any(windows, test))]
use anyhow::{Result, bail};
#[cfg(any(windows, test))]
use std::path::PathBuf;

/// Inspect snapshots for members confirmed before refresh; do not recheck PID membership afterward.
/// Kiểm snapshot của thành viên đã xác nhận trước refresh; không kiểm tra lại membership sau đó.
#[cfg(test)]
pub(in crate::run::implementation::windows_job::inspection::implementation) fn inspect_windows_job_processes_with_shadow(
    root_pid: u32,
    job_process_ids: &[u32],
    processes: &[WindowsProcessSnapshot],
    exempt: &[&str],
    shadow_path: Option<&ShadowPath>,
) -> Result<WindowsJobInspection> {
    inspect_windows_job_processes_with_image_paths(
        root_pid,
        job_process_ids,
        processes,
        exempt,
        shadow_path,
        &std::collections::HashMap::new(),
        None,
    )
}

#[cfg(test)]
pub(in crate::run::implementation::windows_job::inspection::implementation) fn inspect_windows_job_processes_with_image_paths(
    root_pid: u32,
    job_process_ids: &[u32],
    processes: &[WindowsProcessSnapshot],
    exempt: &[&str],
    shadow_path: Option<&ShadowPath>,
    executable_paths: &std::collections::HashMap<u32, PathBuf>,
    compat_audit: Option<&CompatAuditContext<'_>>,
) -> Result<WindowsJobInspection> {
    inspect_windows_job_processes_with_policy_metadata(
        root_pid,
        job_process_ids,
        processes,
        exempt,
        shadow_path,
        &WindowsProcessPolicyMetadata {
            executable_paths,
            go_policy_env: &std::collections::HashMap::new(),
        },
        compat_audit,
        ExecutionScope::Install,
    )
}

/// Observed metadata for confirmed Job members; no parent environment is trusted.
/// Metadata quan sát của thành viên Job; không dùng môi trường cha làm quyền chạy.
#[cfg(any(windows, test))]
#[allow(clippy::too_many_arguments)] // Keep process identity and policy evidence explicit at this boundary.
pub(in crate::run::implementation::windows_job::inspection::implementation) fn inspect_windows_job_processes_with_policy_metadata(
    root_pid: u32,
    job_process_ids: &[u32],
    processes: &[WindowsProcessSnapshot],
    exempt: &[&str],
    shadow_path: Option<&ShadowPath>,
    policy_metadata: &WindowsProcessPolicyMetadata<'_>,
    compat_audit: Option<&CompatAuditContext<'_>>,
    execution_scope: ExecutionScope,
) -> Result<WindowsJobInspection> {
    use std::collections::{HashMap, HashSet};

    let processes_by_pid = processes
        .iter()
        .map(|process| (process.pid, process))
        .collect::<HashMap<_, _>>();
    let mut inspected = WindowsJobInspection::default();
    let mut seen = HashSet::new();
    for &process_id in job_process_ids {
        if process_id == root_pid || !seen.insert(process_id) {
            continue;
        }
        let Some(process) = processes_by_pid.get(&process_id) else {
            bail!("cannot inspect captured Windows job child PID {process_id}");
        };
        let command_line = process
            .command
            .iter()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        let image_name = process.image_name.clone();
        audit_compat_process(compat_audit, process_id, &image_name, &process.command)?;
        if command_line.is_empty() && inspected.missing_command_line.is_none() {
            inspected.missing_command_line = Some((process_id, image_name.clone()));
        }
        let npm_probe_authentication = shadow_path.map(|shadow_path| {
            windows_npm_probe_authentication(
                process,
                policy_metadata
                    .executable_paths
                    .get(&process_id)
                    .map(PathBuf::as_path),
                shadow_path,
            )
        });
        let npm_probe_allowed =
            is_npm_version_shell_probe(process.image_name.as_bytes(), command_line.as_bytes())
                || npm_probe_authentication
                    .as_ref()
                    .is_some_and(|authentication| authentication.is_ok());
        let observed_env = if process_basename(&image_name) == "go" {
            policy_metadata
                .go_policy_env
                .get(&process_id)
                .map(Vec::as_slice)
                .unwrap_or(&[])
        } else {
            &[]
        };
        let forbidden = forbidden_process_match_argv_with_scope(
            &image_name,
            &process.command,
            exempt,
            observed_env,
            execution_scope,
        )
        .filter(|(name, _, _, _)| name != "npm" || !npm_probe_allowed);
        let forbidden_name = forbidden.as_ref().map(|(name, _, _, _)| name.clone());
        if inspected.first_forbidden.is_none()
            && let Some((name, match_source, matched_token, matched_token_index)) = forbidden
        {
            let matched_operation =
                matched_invocation_shape_argv(&name, &process.command, matched_token_index);
            let matched_operation = if name == "npm"
                && matched_operation == "exact read-only probe"
                && let Some(Err(reason)) = npm_probe_authentication
            {
                format!("{matched_operation}; MGC shim authentication rejected: {reason}")
            } else {
                matched_operation
            };
            inspected.first_forbidden = Some(ForbiddenProcess {
                pid: process_id,
                name,
                image_name: Some(image_name),
                match_source,
                matched_token,
                matched_token_index,
                matched_operation,
            });
        }
        inspected.descendants.push((process_id, forbidden_name));
    }
    Ok(inspected)
}
