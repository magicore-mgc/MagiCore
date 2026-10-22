//! Wire the Windows scan snapshot and result types.
//! Nối type snapshot và kết quả scanner Windows.

#[path = "types.rs"]
mod types;
pub(in crate::run::implementation) use types::{WindowsProcessScan, WindowsProcessSnapshot};
