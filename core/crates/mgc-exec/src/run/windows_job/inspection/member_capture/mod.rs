//! Member capture owner facade.
//! Facade owner chụp member.

#[path = "member_capture.rs"]
mod implementation;

#[cfg(test)]
pub(in crate::run::implementation) use implementation::capture_windows_job_members;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::capture_windows_job_process_snapshots;
