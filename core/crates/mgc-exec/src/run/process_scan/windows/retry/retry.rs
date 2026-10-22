//! Retry Windows process metadata scans within the existing deadline.
//! Retry scan metadata process Windows trong deadline hiện tại.

use super::super::WindowsProcessScan;
use super::wait_for_windows_process_retry;
use crate::run::implementation::{
    WINDOWS_COMMAND_LINE_RETRY_DELAY_MS, validate_monitored_child_command_line,
};
use anyhow::{Result, bail};
use std::time::{Duration, Instant};

/// Retry for at most 500 ms of sleep while refreshing Windows child metadata.
/// Chờ tối đa 500 ms tổng thời gian ngủ và liên tục làm mới metadata tiến trình con.
#[cfg(any(windows, test))]
const WINDOWS_COMMAND_LINE_RETRIES: usize = 20;

#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn retry_windows_process_scan(
    started: Instant,
    timeout: Option<Duration>,
    mut child_has_exited: impl FnMut() -> Result<bool>,
    mut inspect_snapshot: impl FnMut() -> Result<WindowsProcessScan>,
) -> Result<WindowsProcessScan> {
    for attempt in 0..=WINDOWS_COMMAND_LINE_RETRIES {
        if child_has_exited()? {
            return Ok(WindowsProcessScan::ChildExited);
        }
        if timeout.is_some_and(|limit| started.elapsed() >= limit) {
            return Ok(WindowsProcessScan::DeadlineReached);
        }
        let snapshot_scan = match inspect_snapshot() {
            Ok(scan) => scan,
            Err(error) => {
                if child_has_exited()? {
                    return Ok(WindowsProcessScan::ChildExited);
                }
                if timeout.is_some_and(|limit| started.elapsed() >= limit) {
                    return Ok(WindowsProcessScan::DeadlineReached);
                }
                return Err(error);
            }
        };
        if !matches!(&snapshot_scan, WindowsProcessScan::Forbidden(_)) {
            if child_has_exited()? {
                return Ok(WindowsProcessScan::ChildExited);
            }
            if timeout.is_some_and(|limit| started.elapsed() >= limit) {
                return Ok(WindowsProcessScan::DeadlineReached);
            }
        }
        match snapshot_scan {
            WindowsProcessScan::Clean => return Ok(WindowsProcessScan::Clean),
            WindowsProcessScan::Forbidden(process) => {
                return Ok(WindowsProcessScan::Forbidden(process));
            }
            WindowsProcessScan::MissingCommandLine { pid, image_name }
                if attempt == WINDOWS_COMMAND_LINE_RETRIES =>
            {
                if child_has_exited()? {
                    return Ok(WindowsProcessScan::ChildExited);
                }
                if timeout.is_some_and(|limit| started.elapsed() >= limit) {
                    return Ok(WindowsProcessScan::DeadlineReached);
                }
                validate_monitored_child_command_line(&image_name, &[], pid)?;
                bail!("command line unexpectedly passed validation for monitored child PID {pid}");
            }
            #[cfg(test)]
            WindowsProcessScan::MissingProcessIdentity { pid }
                if attempt == WINDOWS_COMMAND_LINE_RETRIES =>
            {
                bail!("cannot read process creation identity for monitored PID {pid}");
            }
            WindowsProcessScan::MissingCommandLine { .. } => {
                // Refresh and rewalk the entire tree on the next inspection, not just this PID.
                // Lần kiểm tra sau làm mới và duyệt lại toàn cây, không chỉ hỏi lại PID này.
                let retry_delay = Duration::from_millis(WINDOWS_COMMAND_LINE_RETRY_DELAY_MS);
                if let Some(stop_reason) = wait_for_windows_process_retry(
                    retry_delay,
                    started,
                    timeout,
                    &mut child_has_exited,
                )? {
                    return Ok(stop_reason);
                }
            }
            #[cfg(test)]
            WindowsProcessScan::MissingProcessIdentity { .. } => {
                let retry_delay = Duration::from_millis(WINDOWS_COMMAND_LINE_RETRY_DELAY_MS);
                if let Some(stop_reason) = wait_for_windows_process_retry(
                    retry_delay,
                    started,
                    timeout,
                    &mut child_has_exited,
                )? {
                    return Ok(stop_reason);
                }
            }
            #[cfg(test)]
            WindowsProcessScan::UnverifiableAncestry {
                parent_pid,
                child_pid,
            } => bail!(
                "process ancestry is ambiguous after PID reuse (parent PID {parent_pid}, child PID {child_pid})"
            ),
            WindowsProcessScan::ChildExited | WindowsProcessScan::DeadlineReached => {
                unreachable!("snapshot inspection cannot produce a retry stop reason")
            }
        }
    }
    unreachable!("bounded process metadata retry loop must return")
}
