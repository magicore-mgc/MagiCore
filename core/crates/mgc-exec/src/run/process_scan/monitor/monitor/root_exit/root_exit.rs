//! `root_exit.rs` — inspect descendants after monitored root exit.
//! Kiểm tra descendant sau khi root kết thúc.

use super::super::super::super::super::termination::process_group_id;
use super::super::super::super::super::{
    CompatAuditContext, PROCESS_TREE_TRACKING_ENV, audit_compat_process,
};
use super::super::super::error::MonitoredRootNotVisible;
use super::super::{forbidden_process_entry, is_allowed_shadow_npm_version_probe};
use crate::allowlist::ExecutionScope;
use crate::run::implementation::ForbiddenProcess;
use anyhow::Result;
use std::ffi::OsString;
use std::path::Path;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, UpdateKind};

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn scan_after_root_exit(
    root_pid: u32,
    tracking_token: &str,
    observed_process_groups: &mut std::collections::HashSet<u32>,
    compat_audit: Option<&CompatAuditContext<'_>>,
    exempt: &[&str],
    shadow_dir: &Path,
    execution_scope: ExecutionScope,
) -> Result<Option<ForbiddenProcess>> {
    let mut exit_system = sysinfo::System::new();
    exit_system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cmd(UpdateKind::Always)
            .with_environ(UpdateKind::Always),
    );
    // Reparented children may move into a new session; inspect every group observed
    // while the root was still visible, plus this invocation's inherited marker.
    // Rà group descendant đã ghi nhận và dấu nhận diện kế thừa từ lượt chạy này.
    let mut first_forbidden = None;
    let mut live_descendant_found = false;
    let known_groups = observed_process_groups.clone();
    for (pid, process) in exit_system.processes() {
        if pid.as_u32() == root_pid || process.status() == sysinfo::ProcessStatus::Zombie {
            continue;
        }
        let Some(process_group) = process_group_id(pid.as_u32())? else {
            continue;
        };
        let carries_tracking_token = process.environ().iter().any(|entry| {
            entry == &OsString::from(format!("{PROCESS_TREE_TRACKING_ENV}={tracking_token}"))
        });
        if !known_groups.contains(&process_group) && !carries_tracking_token {
            continue;
        }
        live_descendant_found = true;
        observed_process_groups.insert(process_group);
        if first_forbidden.is_none() {
            audit_compat_process(
                compat_audit,
                pid.as_u32(),
                &process.name().to_string_lossy(),
                process.cmd(),
            )?;
            first_forbidden =
                forbidden_process_entry(&exit_system, pid, process, exempt, execution_scope)
                    .filter(|found| {
                        let allowed = is_allowed_shadow_npm_version_probe(
                            process.name().to_string_lossy().as_bytes(),
                            process
                                .cmd()
                                .iter()
                                .map(|arg| arg.to_string_lossy())
                                .collect::<Vec<_>>()
                                .join(" ")
                                .as_bytes(),
                            shadow_dir,
                            &found.name,
                        );
                        !allowed
                    });
        }
    }
    if let Some(found) = first_forbidden {
        return Ok(Some(found));
    }
    if live_descendant_found {
        return Ok(None);
    }
    Err(anyhow::Error::new(MonitoredRootNotVisible(root_pid)))
}
