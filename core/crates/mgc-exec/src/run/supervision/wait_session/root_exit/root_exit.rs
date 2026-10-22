//! Dispatch root-exit completion to the platform owner.
//! Chuyển xử lý root-exit đến owner của nền tảng.

use super::super::implementation::WaitSession;
use crate::run::implementation::output::ExecOutcome;
use anyhow::Result;

#[cfg(unix)]
impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn finish_root_exit(
        &mut self,
        process_scan: super::super::process_scan::ProcessScan,
    ) -> Result<ExecOutcome> {
        self.finish_unix_root_exit(process_scan)
    }
}

#[cfg(windows)]
impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn finish_root_exit(
        &mut self,
    ) -> Result<ExecOutcome> {
        self.finish_windows_root_exit()
    }
}

#[cfg(not(any(unix, windows)))]
impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn finish_root_exit(
        &mut self,
    ) -> Result<ExecOutcome> {
        Ok(self.output_capture.drain(&mut self.child, None)?.0)
    }
}
