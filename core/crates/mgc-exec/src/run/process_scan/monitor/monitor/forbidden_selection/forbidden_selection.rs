//! `forbidden_selection.rs` — ordered descendant policy audit.
//! Audit policy descendant theo thứ tự.

use super::super::super::super::super::{
    CompatAuditContext, ForbiddenProcess, audit_compat_process,
};
use super::super::{forbidden_process_entry, is_allowed_shadow_npm_version_probe};
use crate::allowlist::ExecutionScope;
use anyhow::Result;
use std::path::Path;

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn find_first_forbidden(
    live_descendant_pids: &[sysinfo::Pid],
    system: &sysinfo::System,
    exempt: &[&str],
    shadow_dir: &Path,
    compat_audit: Option<&CompatAuditContext<'_>>,
    execution_scope: ExecutionScope,
) -> Result<Option<ForbiddenProcess>> {
    let mut first_forbidden = None;
    for pid in live_descendant_pids {
        let Some(process) = system.process(*pid) else {
            continue;
        };
        if first_forbidden.is_none() {
            audit_compat_process(
                compat_audit,
                pid.as_u32(),
                &process.name().to_string_lossy(),
                process.cmd(),
            )?;
            first_forbidden =
                forbidden_process_entry(system, pid, process, exempt, execution_scope).filter(
                    |found| {
                        let command_line = process
                            .cmd()
                            .iter()
                            .map(|arg| arg.to_string_lossy())
                            .collect::<Vec<_>>()
                            .join(" ");
                        let allowed = is_allowed_shadow_npm_version_probe(
                            process.name().to_string_lossy().as_bytes(),
                            command_line.as_bytes(),
                            shadow_dir,
                            &found.name,
                        );
                        !allowed
                    },
                );
        }
    }
    Ok(first_forbidden)
}
