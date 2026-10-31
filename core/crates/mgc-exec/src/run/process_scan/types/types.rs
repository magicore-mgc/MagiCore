//! Shared process identity fields used by OS snapshot adapters and the monitor.
//! Trường định danh process dùng chung cho adapter OS và monitor.

use crate::run::implementation::ForbiddenProcess;

#[cfg(any(unix, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::run::implementation) struct ProcessScanSnapshot {
    pub(in crate::run::implementation::process_scan) parent_pid: Option<u32>,
    pub(in crate::run::implementation::process_scan) process_group: Option<u32>,
    #[cfg(not(target_os = "linux"))]
    pub(in crate::run::implementation::process_scan) start_time_seconds: u64,
    #[cfg(not(target_os = "linux"))]
    pub(in crate::run::implementation::process_scan) start_time_subseconds: u64,
    // The identity helpers below this module read the field, so `pub(super)`
    // was too narrow on Linux; macOS/Windows go through the sibling fields.
    // (Helper định danh bên dưới module này đọc field, nên `pub(super)` quá hẹp
    // trên Linux; macOS/Windows đi qua các field anh em.)
    #[cfg(target_os = "linux")]
    pub(in crate::run::implementation::process_scan) process_start_ticks: u64,
    pub(in crate::run::implementation::process_scan) is_zombie: bool,
}

/// Outcome of one tracked scan while the monitored root may be absent.
/// Kết quả một lượt scan có tracking khi root được giám sát có thể đã biến mất.
#[cfg(unix)]
#[derive(Debug)]
pub(in crate::run::implementation) enum RootExitScan {
    /// A policy violation still runs somewhere in the tracked tree.
    /// Vẫn còn vi phạm policy chạy đâu đó trong cây có tracking.
    Forbidden(ForbiddenProcess),
    /// Tracked descendants are still running but none violates policy.
    /// Descendant có tracking vẫn chạy nhưng chưa vi phạm policy.
    Running,
    /// Root exited and no tracked descendant remains: the tree is complete.
    /// Root đã thoát và không còn descendant: cây process đã kết thúc sạch.
    Exited,
}

#[cfg(all(unix, test))]
impl RootExitScan {
    /// Keep the single-forbidden result shape used by scan callers.
    /// Giữ dạng kết quả một vi phạm mà các caller của scan đang dùng.
    pub(in crate::run::implementation::process_scan) fn into_forbidden(
        self,
    ) -> Option<ForbiddenProcess> {
        match self {
            RootExitScan::Forbidden(found) => Some(found),
            RootExitScan::Running | RootExitScan::Exited => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
