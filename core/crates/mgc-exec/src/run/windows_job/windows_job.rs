//! Windows Job module wiring and non-Windows guard stubs.
//! Wiring module Windows Job và stub guard trên nền tảng không phải Windows.

#[cfg(windows)]
#[path = "deferred_handle_close/mod.rs"]
mod deferred_handle_close;
#[cfg(windows)]
#[path = "job_lifecycle/mod.rs"]
mod job_lifecycle;
#[cfg(windows)]
#[path = "member_capture/mod.rs"]
mod member_capture;
#[cfg(windows)]
#[path = "owned_handle/mod.rs"]
mod owned_handle;
#[cfg(windows)]
#[path = "primary_thread/mod.rs"]
mod primary_thread;
#[cfg(windows)]
#[path = "process_snapshot/mod.rs"]
mod process_snapshot;

#[cfg(windows)]
#[allow(unsafe_code)] // SAFETY proof is on the block below. — Giải trình SAFETY nằm ngay dưới.
pub(in crate::run::implementation::windows_job::implementation) fn windows_last_error() -> u32 {
    // SAFETY: GetLastError reads the calling thread's Win32 error slot.
    unsafe { windows_sys::Win32::Foundation::GetLastError() }
}

#[cfg(windows)]
pub(in crate::run::implementation) use deferred_handle_close::take_deferred_handle_close_error;
#[cfg(windows)]
pub(in crate::run::implementation) use job_lifecycle::ProcessTreeGuard;

#[cfg(unix)]
pub(in crate::run::implementation) struct ProcessTreeGuard;

#[cfg(not(any(unix, windows)))]
pub(in crate::run::implementation) struct ProcessTreeGuard;
