//! Verified Job-member process-handle capture facade.
//! Facade mở handle thành viên Job sau khi xác minh membership.

#[path = "member_capture.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation::windows_job::implementation) use implementation::capture_job_member_process;
