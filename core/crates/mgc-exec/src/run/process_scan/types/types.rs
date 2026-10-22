//! Shared process identity fields used by OS snapshot adapters and the monitor.
//! Trường định danh process dùng chung cho adapter OS và monitor.

#[cfg(any(unix, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::run::implementation) struct ProcessScanSnapshot {
    pub(in crate::run::implementation::process_scan) parent_pid: Option<u32>,
    pub(in crate::run::implementation::process_scan) process_group: Option<u32>,
    #[cfg(not(target_os = "linux"))]
    pub(in crate::run::implementation::process_scan) start_time_seconds: u64,
    #[cfg(not(target_os = "linux"))]
    pub(in crate::run::implementation::process_scan) start_time_subseconds: u64,
    #[cfg(target_os = "linux")]
    pub(super) process_start_ticks: u64,
    pub(in crate::run::implementation::process_scan) is_zombie: bool,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
