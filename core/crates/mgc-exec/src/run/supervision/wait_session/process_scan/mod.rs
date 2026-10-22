//! Unix and Windows process-inspection owners for a wait session.
//! Owner kiểm tra process Unix và Windows trong phiên chờ.

#[path = "process_scan.rs"]
mod implementation;
#[cfg(unix)]
#[path = "unix_active_scan/mod.rs"]
mod unix_active_scan;
#[cfg(unix)]
#[path = "unix_pre_exit_scan/mod.rs"]
mod unix_pre_exit_scan;
#[cfg(windows)]
#[path = "windows_active_scan/mod.rs"]
mod windows_active_scan;

pub(super) use implementation::ProcessScan;
