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

// One export per name: two cfg blocks naming the same item collide on the
// platform where both apply.
// (Mỗi tên một export: hai block cfg nêu cùng item sẽ xung đột trên nền tảng
// mà cả hai đều áp.)
#[cfg(test)]
pub(in crate::run::implementation) use ancestry::{
    WindowsProcessAncestry, scan_windows_process_snapshot,
};
pub(in crate::run::implementation) use retry::retry_windows_process_scan;
pub(in crate::run::implementation) use types::{WindowsProcessScan, WindowsProcessSnapshot};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
