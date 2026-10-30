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
    // The identity helpers below this module read the field, so `pub(super)`
    // was too narrow on Linux; macOS/Windows go through the sibling fields.
    // (Helper định danh bên dưới module này đọc field, nên `pub(super)` quá hẹp
    // trên Linux; macOS/Windows đi qua các field anh em.)
    #[cfg(target_os = "linux")]
    pub(in crate::run::implementation::process_scan) process_start_ticks: u64,
    pub(in crate::run::implementation::process_scan) is_zombie: bool,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
