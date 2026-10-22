//! Wire Windows process scan owners and preserve the feature's existing exports.
//! Nối owner scanner process Windows và giữ nguyên export của feature.

#[cfg(test)]
#[path = "ancestry/mod.rs"]
mod ancestry;
#[path = "retry/mod.rs"]
mod retry;
#[cfg(all(windows, test))]
#[path = "system_snapshot/mod.rs"]
mod system_snapshot;
#[path = "types/mod.rs"]
mod types;

#[cfg(test)]
pub(super) use ancestry::{WindowsProcessAncestry, scan_windows_process_snapshot};
pub(in crate::run::implementation) use retry::retry_windows_process_scan;
#[cfg(all(windows, test))]
pub(super) use system_snapshot::find_forbidden_descendant_with_timeout;
pub(in crate::run::implementation) use types::{WindowsProcessScan, WindowsProcessSnapshot};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
