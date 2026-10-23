//! Process-wide shutdown flag and checked OS signal-handler installation.
//! Cờ shutdown toàn process và cài signal handler có kiểm tra lỗi theo OS.

use super::error::ShutdownError;
use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
};

/// Shared flag read by execution supervision and written by handlers/embedders.
/// Cờ dùng chung cho supervision và signal handler/embedder.
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static SIGNAL_HANDLER_INSTALLATION: OnceLock<std::result::Result<(), ShutdownError>> =
    OnceLock::new();

/// Install async-signal-safe shutdown handlers once; return registration errors before spawn.
/// Cài handler shutdown async-signal-safe một lần; trả lỗi để caller chặn spawn.
pub fn ensure_signal_handlers() -> anyhow::Result<()> {
    let result = *SIGNAL_HANDLER_INSTALLATION.get_or_init(install_signal_handlers);
    result.map_err(anyhow::Error::new)
}

fn install_signal_handlers() -> std::result::Result<(), ShutdownError> {
    #[cfg(unix)]
    install_unix_signal_handlers()?;
    #[cfg(windows)]
    install_windows_console_handler()?;
    Ok(())
}

#[cfg(unix)]
fn install_unix_signal_handlers() -> std::result::Result<(), ShutdownError> {
    // SAFETY: installing a C signal handler is a process-wide FFI operation;
    // the handler only performs an atomic store (async-signal-safe) and the
    // signal numbers come from libc constants, so no Rust invariants are
    // violated.
    // AN TOÀN: cài signal handler C là thao tác FFI toàn process; handler chỉ
    // store atomic (async-signal-safe) và số hiệu signal lấy từ libc.
    #[allow(unsafe_code)]
    unsafe {
        extern "C" fn flag_shutdown(_signal: libc::c_int) {
            SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
        }

        let interrupt = libc::signal(
            libc::SIGINT,
            flag_shutdown as *const () as libc::sighandler_t,
        );
        let interrupt_registered = interrupt != libc::SIG_ERR;
        let interrupt_error_code = if interrupt_registered {
            None
        } else {
            std::io::Error::last_os_error().raw_os_error()
        };
        ShutdownError::check_signal_handler_registration(
            "SIGINT",
            interrupt_registered,
            interrupt_error_code,
        )?;

        let terminate = libc::signal(
            libc::SIGTERM,
            flag_shutdown as *const () as libc::sighandler_t,
        );
        let terminate_registered = terminate != libc::SIG_ERR;
        let terminate_error_code = if terminate_registered {
            None
        } else {
            std::io::Error::last_os_error().raw_os_error()
        };
        ShutdownError::check_signal_handler_registration(
            "SIGTERM",
            terminate_registered,
            terminate_error_code,
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn install_windows_console_handler() -> std::result::Result<(), ShutdownError> {
    // SAFETY: SetConsoleCtrlHandler receives a Windows console callback; the
    // callback body only performs an atomic store and returns TRUE, which is
    // the documented contract for handler routines.
    // AN TOÀN: SetConsoleCtrlHandler nhận callback console Windows; thân
    // callback chỉ store atomic và trả TRUE đúng contract của handler.
    #[allow(unsafe_code)]
    unsafe {
        unsafe extern "system" fn flag_shutdown_console(_ctrl: u32) -> i32 {
            SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
            windows_sys::Win32::Foundation::TRUE
        }

        let installed = windows_sys::Win32::System::Console::SetConsoleCtrlHandler(
            Some(flag_shutdown_console),
            windows_sys::Win32::Foundation::TRUE,
        );
        let error_code = if installed == 0 {
            Some(windows_sys::Win32::Foundation::GetLastError())
        } else {
            None
        };
        ShutdownError::check_console_handler_registration(installed != 0, error_code)?;
    }
    Ok(())
}

/// Set a shutdown request directly for embedders and tests.
/// Cho embedder và test bật cờ shutdown trực tiếp.
pub fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
}

/// Clear the flag after the owning supervisor has reaped its process tree.
/// Xóa cờ sau khi supervisor đã thu hồi cây process.
pub fn clear_shutdown() {
    SHUTDOWN_REQUESTED.store(false, Ordering::SeqCst);
}

/// Return whether a signal or direct request has asked the process to stop.
/// Trả về trạng thái có yêu cầu dừng từ signal hoặc caller trực tiếp.
pub fn shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
