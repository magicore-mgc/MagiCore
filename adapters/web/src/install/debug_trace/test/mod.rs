//! Verifies the installer debug gate only reports when explicitly enabled.
//! Kiểm cổng debug installer chỉ báo khi được bật tường minh.

use super::INSTALL_DEBUG_ENV;
use super::install_debug_enabled;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_debug_env<T>(value: Option<&str>, body: impl FnOnce() -> T) -> T {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = std::env::var_os(INSTALL_DEBUG_ENV);
    match value {
        Some(raw) =>
        // SAFETY: single-threaded test mutation under ENV_LOCK.
        // AN TOÀN: test ghi state đơn luồng dưới ENV_LOCK.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(INSTALL_DEBUG_ENV, raw)
            };
        }
        None =>
        // SAFETY: single-threaded test mutation under ENV_LOCK.
        // AN TOÀN: test ghi state đơn luồng dưới ENV_LOCK.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(INSTALL_DEBUG_ENV)
            };
        }
    }
    let result = body();
    match previous {
        Some(value) =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(INSTALL_DEBUG_ENV, value)
            };
        }
        None =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(INSTALL_DEBUG_ENV)
            };
        }
    }
    result
}

#[test]
fn debug_traces_are_off_when_the_flag_is_absent() {
    with_debug_env(None, || assert!(!install_debug_enabled()));
}

#[test]
fn blank_and_false_spellings_keep_traces_off() {
    for value in ["", "0", "false", "FALSE"] {
        with_debug_env(Some(value), || {
            assert!(!install_debug_enabled(), "value {value:?} must stay off");
        });
    }
}

#[test]
fn a_truthy_flag_turns_traces_on() {
    for value in ["1", "true", "yes", "on"] {
        with_debug_env(Some(value), || {
            assert!(install_debug_enabled(), "value {value:?} must turn on");
        });
    }
}
