//! Member capture owner facade.
//! Facade owner chụp member.

#[path = "member_capture.rs"]
mod implementation;

#[cfg(test)]
pub(super) use implementation::capture_windows_job_members;
#[cfg(any(windows, test))]
pub(super) use implementation::capture_windows_job_process_snapshots;
