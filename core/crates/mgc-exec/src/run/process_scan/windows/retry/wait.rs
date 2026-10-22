//! Poll process exit and deadline during a bounded metadata retry.
//! Poll trạng thái process và deadline trong lượt retry metadata có giới hạn.

use super::super::WindowsProcessScan;
use anyhow::Result;
use std::time::{Duration, Instant};

/// Check child status during short waits so natural exits stay responsive.
/// Kiểm tra trạng thái child trong lúc chờ ngắn để nhận biết thoát tự nhiên kịp thời.
#[cfg(any(windows, test))]
const WINDOWS_COMMAND_LINE_RETRY_POLL_INTERVAL_MS: u64 = 1;

#[cfg(any(windows, test))]
pub(in crate::run::implementation::process_scan::windows::implementation) fn wait_for_windows_process_retry(
    retry_delay: Duration,
    started: Instant,
    timeout: Option<Duration>,
    child_has_exited: &mut impl FnMut() -> Result<bool>,
) -> Result<Option<WindowsProcessScan>> {
    let wait_started = Instant::now();
    loop {
        if child_has_exited()? {
            return Ok(Some(WindowsProcessScan::ChildExited));
        }
        if timeout.is_some_and(|limit| started.elapsed() >= limit) {
            return Ok(Some(WindowsProcessScan::DeadlineReached));
        }

        let wait_elapsed = wait_started.elapsed();
        if wait_elapsed >= retry_delay {
            return Ok(None);
        }
        let poll_interval = Duration::from_millis(WINDOWS_COMMAND_LINE_RETRY_POLL_INTERVAL_MS);
        let remaining_wait = retry_delay.saturating_sub(wait_elapsed);
        let remaining_timeout = timeout
            .map(|limit| limit.saturating_sub(started.elapsed()))
            .unwrap_or(remaining_wait);
        let sleep_for = remaining_wait.min(poll_interval).min(remaining_timeout);
        if sleep_for.is_zero() {
            return Ok(Some(WindowsProcessScan::DeadlineReached));
        }
        std::thread::sleep(sleep_for);
    }
}
