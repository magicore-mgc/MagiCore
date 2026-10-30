//! Wire Windows PID identity state and ancestry traversal owners.
//! Nối owner state định danh PID và duyệt ancestry Windows.

#[path = "command_scan.rs"]
mod command_scan;
#[path = "identity.rs"]
mod identity;
#[path = "scan.rs"]
mod scan;
#[cfg(test)]
pub(in crate::run::implementation) use identity::WindowsProcessAncestry;
#[cfg(test)]
pub(in crate::run::implementation) use scan::scan_windows_process_snapshot;
