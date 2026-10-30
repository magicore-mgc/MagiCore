//! Conversion of Job observations into shared process-scan outcomes.
//! Chuyển quan sát Job thành kết quả process scan dùng chung.

#[cfg(any(windows, test))]
use super::super::super::super::super::process_scan::WindowsProcessScan;
#[cfg(windows)]
use super::super::super::super::super::{CompatAuditContext, ShadowPath};
#[cfg(windows)]
use super::super::super::super::ProcessTreeGuard;
#[cfg(windows)]
use super::super::super::error::InspectionError;
#[cfg(windows)]
use super::super::job_inspection::inspect_windows_job;
#[cfg(any(windows, test))]
use super::super::{ForbiddenProcess, WindowsJobInspection};
#[cfg(windows)]
use crate::allowlist::ExecutionScope;
#[cfg(windows)]
use crate::run::implementation::process_scan::retry_windows_process_scan;
#[cfg(windows)]
use anyhow::Result;
#[cfg(windows)]
use std::time::{Duration, Instant};

#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn windows_job_has_active_descendants(
    root_pid: u32,
    process_ids: &[u32],
) -> bool {
    process_ids.iter().any(|process_id| *process_id != root_pid)
}

#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn windows_job_scan_from_inspection(
    inspection: WindowsJobInspection,
) -> WindowsProcessScan {
    if let Some(forbidden) = inspection.first_forbidden {
        return WindowsProcessScan::Forbidden(forbidden);
    }
    if let Some((pid, Some(name))) = inspection
        .descendants
        .into_iter()
        .find(|(_, forbidden)| forbidden.is_some())
    {
        return WindowsProcessScan::Forbidden(ForbiddenProcess {
            pid,
            name,
            image_name: None,
            match_source: "Windows job scan",
            matched_token: None,
            matched_token_index: None,
            matched_operation: "unavailable".to_string(),
        });
    }
    if let Some((pid, image_name)) = inspection.missing_command_line {
        return WindowsProcessScan::MissingCommandLine { pid, image_name };
    }
    WindowsProcessScan::Clean
}

/// Inspect native Windows process metadata, without a shell or WMI command.
/// Kiểm metadata process Windows bằng API native, không shell hoặc lệnh WMI.
#[cfg(windows)]
pub(in crate::run::implementation) fn find_forbidden_job_descendant_with_timeout(
    guard: &ProcessTreeGuard,
    root_pid: u32,
    exempt: &[&str],
    shadow_path: &ShadowPath,
    compat_audit: Option<&CompatAuditContext<'_>>,
    started: Instant,
    timeout: Option<Duration>,
    execution_scope: ExecutionScope,
    child_has_exited: impl FnMut() -> Result<bool>,
) -> std::result::Result<WindowsProcessScan, InspectionError> {
    retry_windows_process_scan(started, timeout, child_has_exited, || {
        Ok(windows_job_scan_from_inspection(
            inspect_windows_job(
                guard,
                root_pid,
                exempt,
                Some(shadow_path),
                compat_audit,
                execution_scope,
            )
            .map_err(anyhow::Error::new)?,
        ))
    })
    .map_err(InspectionError::RetryScan)
}
