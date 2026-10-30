//! Process-scan facade for shared types, Unix monitoring and OS snapshot adapters.
//! Facade process-scan cho type dùng chung, monitor Unix và adapter snapshot theo OS.

#[cfg(any(unix, test))]
#[path = "types/mod.rs"]
mod types;
#[cfg(any(unix, test))]
// Consumers live under `run::implementation::process_scan::monitor`, one level
// below this module, so the re-export must reach `run::implementation` — the
// same visibility the monitor items use. On macOS the field-level access below
// compiled anyway, which is why only Linux builds caught it.
// (Consumer nằm dưới `run::implementation::process_scan::monitor`, thấp hơn
// module này một mức, nên re-export phải tới `run::implementation` — đúng
// visibility mà các item monitor dùng. Trên macOS field-level access vẫn biên
// dịch được, nên chỉ build Linux mới bắt được.)
pub(in crate::run::implementation) use types::ProcessScanSnapshot;

#[cfg(any(unix, test))]
mod monitor;
#[cfg(unix)]
pub(in crate::run::implementation) use monitor::{
    MonitoredRootNotVisible, find_forbidden_descendant_tracking,
};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod linux;

#[cfg(target_os = "linux")]
pub(in crate::run::implementation) use linux::{
    linux_process_snapshots, linux_process_stat_snapshot,
};

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod macos;

#[cfg(target_os = "macos")]
pub(in crate::run::implementation) use macos::{macos_process_snapshot, macos_process_snapshots};

#[cfg(any(windows, test))]
#[path = "windows/mod.rs"]
mod windows;

#[cfg(any(windows, test))]
pub(super) use windows::{WindowsProcessScan, WindowsProcessSnapshot};
