//! Scan result owner facade.
//! Facade owner kết quả scan.

#[path = "scan_result.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    find_forbidden_job_descendant_with_timeout, windows_job_has_active_descendants,
};
// The test build reaches the same items as the Windows build, so this hop must
// not be narrower than the one above it.
// (Build test cũng chạm các item như build Windows, nên hop này không được hẹp
// hơn hop phía trên.)
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::{
    windows_job_has_active_descendants, windows_job_scan_from_inspection,
};
