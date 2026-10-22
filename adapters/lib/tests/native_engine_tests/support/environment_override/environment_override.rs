//! Scope test-only environment values and restore the exact previous state.
//! Giới hạn env chỉ trong test và khôi phục đúng trạng thái ban đầu.

use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static ENVIRONMENT_ACCESS: Mutex<()> = Mutex::new(());

/// Keep environment-mutating tests from racing inside this integration target.
/// Giữ các test sửa env không chạy đua trong integration target này.
pub(crate) struct EnvironmentAccessGuard {
    _lock: MutexGuard<'static, ()>,
}

/// Serialize Lib integration tests which read or mutate process environment.
/// Tuần tự hóa test integration Lib có đọc hoặc sửa environment process.
pub(crate) fn lock_environment_access() -> EnvironmentAccessGuard {
    let lock = ENVIRONMENT_ACCESS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    EnvironmentAccessGuard { _lock: lock }
}

/// Restore each captured key to its original value or absence when dropped.
/// Khi drop, trả từng key về giá trị OsString cũ hoặc trạng thái vắng mặt.
pub(crate) struct EnvironmentOverrideGuard<'a> {
    originals: Vec<(&'static str, Option<OsString>)>,
    _access: &'a EnvironmentAccessGuard,
}

impl<'a> EnvironmentOverrideGuard<'a> {
    /// Set scoped test values while retaining their original process state.
    /// Gán env scoped của test và giữ lại trạng thái process ban đầu.
    pub(crate) fn set(
        access: &'a EnvironmentAccessGuard,
        values: &[(&'static str, String)],
    ) -> Self {
        let keys = values.iter().map(|(key, _)| *key).collect::<Vec<_>>();
        let guard = Self::capture(access, &keys);
        // SAFETY: `access` holds the target-wide process-environment mutex.
        // AN TOÀN: `access` đang giữ mutex environment của toàn target test.
        unsafe {
            for (key, value) in values {
                std::env::set_var(key, value);
            }
        }
        guard
    }

    /// Temporarily remove keys so tests can verify absence restoration.
    /// Tạm xóa key để test kiểm tra khôi phục trạng thái không có giá trị.
    pub(crate) fn remove(access: &'a EnvironmentAccessGuard, keys: &[&'static str]) -> Self {
        let guard = Self::capture(access, keys);
        // SAFETY: `access` holds the target-wide process-environment mutex.
        // AN TOÀN: `access` đang giữ mutex environment của toàn target test.
        unsafe {
            for key in keys {
                std::env::remove_var(key);
            }
        }
        guard
    }

    fn capture(access: &'a EnvironmentAccessGuard, keys: &[&'static str]) -> Self {
        let mut originals: Vec<(&'static str, Option<OsString>)> = Vec::with_capacity(keys.len());
        for key in keys {
            if originals.iter().any(|(captured, _)| captured == key) {
                continue;
            }
            originals.push((key, std::env::var_os(key)));
        }
        Self {
            originals,
            _access: access,
        }
    }
}

impl Drop for EnvironmentOverrideGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: the borrowed access guard still holds the target-wide mutex.
        // AN TOÀN: access guard được mượn vẫn giữ mutex toàn target.
        unsafe {
            for (key, original) in self.originals.drain(..) {
                match original {
                    Some(value) => std::env::set_var(key, value),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}

#[path = "test/mod.rs"]
mod tests;
pub(crate) use tests::environment_override_restores_preexisting_and_absent_values;
