//! Windows Job facade — export lifecycle, inspection, and cleanup boundaries.
//! Facade Windows Job — export vòng đời, kiểm tra và ranh giới cleanup.

#[cfg(windows)]
#[path = "error.rs"]
mod error;
#[path = "windows_job.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::ProcessTreeGuard;
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::take_deferred_handle_close_error;

#[cfg(any(windows, test))]
mod inspection;
#[cfg(windows)]
mod root_exit;

#[cfg(windows)]
pub(in crate::run::implementation) use inspection::find_forbidden_job_descendant_with_timeout;
#[cfg(windows)]
pub(in crate::run::implementation) use root_exit::finish_monitored_root_exit;
