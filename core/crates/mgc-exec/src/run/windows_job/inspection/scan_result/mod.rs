//! Scan result owner facade.
//! Facade owner kết quả scan.

#[path = "scan_result.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::find_forbidden_job_descendant_with_timeout;
#[cfg(any(windows, test))]
pub(in crate::run::implementation::windows_job) use implementation::{
    windows_job_has_active_descendants, windows_job_scan_from_inspection,
};
