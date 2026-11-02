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
// The npm-probe suite addresses this entry point as
// `run::implementation::inspect_windows_job_processes_with_image_paths`, so the
// chain has to forward it at every level: inspection facade -> windows_job
// facade -> run facade. Stopping at `inspection` made the Windows test build
// fail with E0432 while macOS never compiled the branch.
// (Bộ test npm-probe gọi entry point này qua
// `run::implementation::inspect_windows_job_processes_with_image_paths`, nên
// chuỗi phải chuyển tiếp ở mọi cấp: facade inspection -> facade windows_job ->
// facade run. Dừng ở `inspection` khiến build test Windows lỗi E0432 trong khi
// macOS không bao giờ biên dịch nhánh đó.)
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use inspection::inspect_windows_job_processes_with_image_paths;
#[cfg(windows)]
pub(in crate::run::implementation) use root_exit::finish_monitored_root_exit;
