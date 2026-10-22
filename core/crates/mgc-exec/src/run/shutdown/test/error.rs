//! Tests for typed signal-handler errors and native source retention.
//! Test lỗi typed signal-handler và giữ source native.

use crate::run::implementation::shutdown::error::ShutdownError;

#[cfg(unix)]
#[test]
fn unix_handler_failure_retains_signal_name_and_native_source() {
    let error = ShutdownError::check_signal_handler_registration("SIGTERM", false, Some(22))
        .expect_err("failed signal registration must produce a typed error");
    assert!(error.to_string().contains("SIGTERM"));
    let source = std::error::Error::source(&error)
        .expect("signal registration error must retain its native source");
    assert!(source.to_string().contains("libc::signal"));
    assert!(source.to_string().contains("22"));
}

#[cfg(unix)]
#[test]
fn unix_handler_success_is_not_reported_as_failure() {
    assert!(ShutdownError::check_signal_handler_registration("SIGINT", true, None).is_ok());
}

#[cfg(unix)]
#[test]
fn unix_handler_failure_does_not_fabricate_a_native_error_code() {
    let error = ShutdownError::check_signal_handler_registration("SIGTERM", false, None)
        .expect_err("failed signal registration must still be surfaced without errno");
    let source = std::error::Error::source(&error)
        .expect("signal registration error must retain its native operation");
    assert!(source.to_string().contains("code unavailable"));
}

#[cfg(windows)]
#[test]
fn console_handler_failure_retains_native_source() {
    let error = ShutdownError::check_console_handler_registration(false, Some(5))
        .expect_err("failed console registration must produce a typed error");
    assert!(error.to_string().contains("Windows console"));
    let source = std::error::Error::source(&error)
        .expect("console handler registration error must retain its native source");
    assert!(source.to_string().contains("SetConsoleCtrlHandler"));
    assert!(source.to_string().contains("5"));
}

#[cfg(windows)]
#[test]
fn console_handler_success_is_not_reported_as_failure() {
    assert!(ShutdownError::check_console_handler_registration(true, None).is_ok());
}
