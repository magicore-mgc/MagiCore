//! Windows Job root-exit completion and tree cleanup.
//! Hoàn tất root process và dọn cây process Windows Job.

use super::super::super::output::ExecOutcome;
use super::super::super::{
    CompatAuditContext, ForbiddenProcess, NativeAuditContext, POST_KILL_DRAIN_MS, ShadowPath,
    forbidden_child_error,
};
use super::super::ProcessTreeGuard;
use super::super::inspection::{windows_job_descendants, windows_job_has_active_descendants};
use super::error::RootExitError;
#[cfg(windows)]
use crate::allowlist::ExecutionScope;
use anyhow::{Context, Result};
use std::time::Duration;

#[cfg(windows)]
pub(super) fn finish_monitored_root_exit(
    child: &mut std::process::Child,
    guard: ProcessTreeGuard,
    exempt: &[&str],
    shadow_path: &ShadowPath,
    compat_audit: Option<&CompatAuditContext<'_>>,
    native_audit: Option<&NativeAuditContext<'_>>,
    execution_scope: ExecutionScope,
    drain: impl Fn(&mut std::process::Child, Option<Duration>) -> Result<(ExecOutcome, bool)>,
) -> std::result::Result<ExecOutcome, RootExitError> {
    let survivors = windows_job_descendants(
        &guard,
        child.id(),
        exempt,
        shadow_path,
        compat_audit,
        execution_scope,
    )
    .map_err(anyhow::Error::new)
    .map_err(RootExitError::DescendantInspection)?;
    if survivors.is_empty() {
        let active_process_ids = guard
            .active_process_ids()
            .map_err(RootExitError::ActiveMemberEnumeration)?;
        if windows_job_has_active_descendants(child.id(), &active_process_ids) {
            let termination = guard.terminate();
            drop(guard);
            let (out, _) = drain(child, Some(Duration::from_millis(POST_KILL_DRAIN_MS)))
                .map_err(RootExitError::OutputDrain)?;
            if let Err(error) = termination {
                return Err(RootExitError::Termination {
                    phase: "late-child cleanup",
                    source: anyhow::Error::new(error)
                        .context("failed to terminate late Windows job descendants"),
                });
            }
            let child_ids = active_process_ids
                .iter()
                .copied()
                .filter(|process_id| *process_id != child.id())
                .map(|process_id| process_id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(RootExitError::UninspectedChildren {
                process_ids: child_ids,
                status: out.status.to_string(),
            });
        }
        drop(guard);
        return Ok(
            drain(child, Some(Duration::from_millis(POST_KILL_DRAIN_MS)))
                .map_err(RootExitError::OutputDrain)?
                .0,
        );
    }

    let termination = guard.terminate();
    drop(guard);
    let (out, _) = drain(child, Some(Duration::from_millis(POST_KILL_DRAIN_MS)))
        .map_err(RootExitError::OutputDrain)?;
    if let Some((pid, Some(name))) = survivors.iter().find(|(_, forbidden)| forbidden.is_some()) {
        let source = forbidden_child_error(
            &ForbiddenProcess {
                pid: *pid,
                name: name.clone(),
                image_name: None,
                match_source: "process-table scan",
                matched_token: None,
                matched_token_index: None,
                matched_operation: "unavailable".to_string(),
            },
            out.status,
            &out.stderr,
            &out.stdout,
            native_audit,
        );
        return Err(RootExitError::ForbiddenChild(anyhow::Error::new(source)));
    }
    if let Err(error) = termination {
        return Err(RootExitError::Termination {
            phase: "surviving-child cleanup",
            source: anyhow::Error::new(error)
                .context("failed to terminate surviving Windows child processes"),
        });
    }
    let child_ids = survivors
        .iter()
        .map(|(pid, _)| pid.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    Err(RootExitError::SurvivingChildren {
        process_ids: child_ids,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
