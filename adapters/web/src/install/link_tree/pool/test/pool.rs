//! Verify bounded Rayon pool configuration.
//! Kiểm tra cấu hình Rayon pool có giới hạn.

#![allow(unsafe_code)]

use std::ffi::OsString;
use std::sync::MutexGuard;

use super::super::implementation::{default_hardlink_threads, hardlink_thread_count};

struct EnvGuard {
    key: &'static str,
    previous: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

impl EnvGuard {
    fn new(key: &'static str) -> Self {
        let lock = crate::environment_test_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        Self {
            key,
            previous: std::env::var_os(key),
            _lock: lock,
        }
    }

    fn set(&self, value: Option<&str>) {
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe {
            match value {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe {
            match self.previous.take() {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[test]
fn thread_count_override_and_bounded_default_are_stable() {
    let env = EnvGuard::new("MAGICORE_WEB_HARDLINK_THREADS");
    let default = default_hardlink_threads();
    assert!((1..=6).contains(&default));

    env.set(Some(" 4 "));
    assert_eq!(hardlink_thread_count(), 4);
    env.set(Some("0"));
    assert_eq!(hardlink_thread_count(), default);
    env.set(Some("invalid"));
    assert_eq!(hardlink_thread_count(), default);
}
