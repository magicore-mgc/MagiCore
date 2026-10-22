//! Process-scan facade for shared types, Unix monitoring and OS snapshot adapters.
//! Facade process-scan cho type dùng chung, monitor Unix và adapter snapshot theo OS.

#[cfg(any(unix, test))]
#[path = "types/mod.rs"]
mod types;
#[cfg(any(unix, test))]
pub(super) use types::ProcessScanSnapshot;

#[cfg(any(unix, test))]
mod monitor;
#[cfg(unix)]
pub(super) use monitor::{MonitoredRootNotVisible, find_forbidden_descendant_tracking};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod linux;

#[cfg(target_os = "linux")]
pub(super) use linux::{linux_process_snapshots, linux_process_stat_snapshot};

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod macos;

#[cfg(target_os = "macos")]
pub(super) use macos::{macos_process_snapshot, macos_process_snapshots};

#[cfg(any(windows, test))]
#[path = "windows/mod.rs"]
mod windows;

#[cfg(any(windows, test))]
pub(super) use windows::{WindowsProcessScan, WindowsProcessSnapshot};
