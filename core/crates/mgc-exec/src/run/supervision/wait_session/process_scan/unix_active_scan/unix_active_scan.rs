//! Retry transient root-exit snapshots and classify process findings.
//! Thử lại snapshot root đã thoát tạm thời và phân loại kết quả scan.

use super::super::super::implementation::WaitSession;
use super::super::ProcessScan;
use crate::run::implementation::{
    RootExitScan, WAIT_POLL_INTERVAL_MS, find_forbidden_descendant_tracking,
};
use anyhow::Result;
use std::time::Duration;

/// Retry an early root-exit snapshot for 100 ms before trusting it.
/// Thử lại snapshot root thoát sớm trong 100 ms trước khi tin kết quả.
#[cfg(unix)]
const ROOT_VISIBILITY_RETRIES: usize = 5;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn inspect_unix_processes(
        &mut self,
        mut process_scan: ProcessScan,
    ) -> Result<()> {
        if self.scan_reports_early_root_exit(&process_scan) {
            self.retry_early_unix_root_exit(&mut process_scan);
        }

        match process_scan {
            Ok(RootExitScan::Forbidden(found)) => Err(self.fail_forbidden_process(&found, true)),
            Ok(RootExitScan::Exited) if self.child.try_wait()?.is_some() => {
                // The child is reaped; the next poll evaluates the clean exit.
                // (Child đã được reap; poll kế tiếp sẽ xét cây đã kết thúc sạch.)
                self.root_reaped = true;
                Ok(())
            }
            Ok(RootExitScan::Exited) => {
                // A visible, still-running root cannot be a completed tree; treat
                // the snapshot as transient and make no policy decision from it.
                // (Root vẫn sống và nhìn thấy được thì không thể là cây đã xong;
                // coi snapshot này là tạm thời và không quyết định policy từ nó.)
                Ok(())
            }
            Ok(RootExitScan::Running) => Ok(()),
            Err(error) => Err(self.fail_process_inspection(
                error,
                "during command execution",
                "failing closed after process inspection failed",
                "failing closed after process inspection failed",
            )),
        }
    }

    fn scan_reports_early_root_exit(&self, process_scan: &ProcessScan) -> bool {
        matches!(process_scan, Ok(RootExitScan::Exited))
    }

    fn retry_early_unix_root_exit(&mut self, process_scan: &mut ProcessScan) {
        for attempt in 0..ROOT_VISIBILITY_RETRIES {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    // A reaped root lets the exit rules decide: they track the
                    // surviving process groups instead of the root.
                    // (Root đã reap để quy tắc sau exit quyết định: theo dõi process
                    // group còn sống thay vì root.)
                    self.root_reaped = true;
                    *process_scan = self.scan_tracked_unix_descendants();
                    // A cleared group set on a reaped root is conclusive; more
                    // retries would only repeat the same read.
                    // (Tập group rỗng khi root đã reap là kết luận cuối; thử lại chỉ
                    // đọc lặp cùng một kết quả.)
                    if !self.scan_reports_early_root_exit(process_scan)
                        || attempt + 1 == ROOT_VISIBILITY_RETRIES
                    {
                        return;
                    }
                }
                Ok(None) => {
                    std::thread::sleep(Duration::from_millis(WAIT_POLL_INTERVAL_MS));
                    *process_scan = self.scan_tracked_unix_descendants();
                    if !self.scan_reports_early_root_exit(process_scan) {
                        return;
                    }
                }
                Err(error) => {
                    *process_scan = Err(anyhow::Error::from(error));
                    return;
                }
            }
        }
    }

    fn scan_tracked_unix_descendants(&mut self) -> Result<RootExitScan> {
        find_forbidden_descendant_tracking(
            self.child.id(),
            &mut self.process_system,
            &mut self.monitored_process_pids,
            self.process_monitor.exempt,
            self.process_monitor.shadow_dir,
            self.process_monitor.tracking_token,
            &mut self.observed_process_groups,
            self.process_monitor.compat_audit,
            self.root_reaped,
            self.process_monitor.execution_scope,
        )
    }
}
