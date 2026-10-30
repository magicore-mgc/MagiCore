//! Scan result owner facade.
//! Facade owner kết quả scan.

#[path = "scan_result.rs"]
mod implementation;

// One export per item: exporting the same name under two `cfg` blocks gave a
// Windows build two definitions of it.
// (Mỗi item export đúng một lần: export cùng tên trong hai block `cfg` khiến
// build Windows thấy hai định nghĩa.)
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::find_forbidden_job_descendant_with_timeout;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::{
    windows_job_has_active_descendants, windows_job_scan_from_inspection,
};
