//! Shared fixtures and process-environment guards for lifecycle unit tests.
//! Fixture chung và guard biến môi trường process cho test lifecycle.

use std::ffi::OsString;
use std::sync::MutexGuard;

/// SEC-02 moved bare `python3` out of the install scope, and a project-local
/// executable is allowed only under an absolute path. Tests therefore address
/// the host interpreter explicitly, exactly like a real project would.
/// (SEC-02 đưa `python3` trần ra khỏi scope install; executable cục bộ project
/// chỉ được dùng qua đường dẫn tuyệt đối. Vì vậy test gọi interpreter bằng
/// đường dẫn tuyệt đối, giống project thật.)
pub(super) fn write_package_script(package: &std::path::Path, script: &str) {
    let manifest = serde_json::json!({
        "scripts": {
            "postinstall": script,
        }
    });
    std::fs::write(package.join("package.json"), manifest.to_string()).unwrap();
}

pub(super) struct ScopedEnvironmentVariable {
    key: &'static str,
    previous: Option<OsString>,
    _guard: MutexGuard<'static, ()>,
}

impl ScopedEnvironmentVariable {
    pub(super) fn set(key: &'static str, value: &str) -> Self {
        // Poisoning-free lock: a test that asserts a failure while holding it
        // must not break the rest of the binary.
        // (Lock không nhiễm độc: test assert lỗi khi giữ lock không được làm
        // hỏng phần còn lại của binary.)
        let guard = crate::environment_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous = std::env::var_os(key);
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            previous,
            _guard: guard,
        }
    }
}

impl Drop for ScopedEnvironmentVariable {
    fn drop(&mut self) {
        if let Some(value) = self.previous.take() {
            // SAFETY: test-only env mutation; callers hold `env_lock` so no other
            // test thread observes a partially written environment.
            // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
            // không thấy môi trường ghi dở.
            // SAFETY: test-only operation on process-local state / FFI with
            // caller-owned buffers kept alive for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
            // buffer do caller sở hữu và còn sống trong lời gọi.
            unsafe { std::env::set_var(self.key, value) };
        } else {
            // SAFETY: test-only env mutation; callers hold `env_lock` so no other
            // test thread observes a partially written environment.
            // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
            // không thấy môi trường ghi dở.
            // SAFETY: test-only operation on process-local state / FFI with
            // caller-owned buffers kept alive for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
            // buffer do caller sở hữu và còn sống trong lời gọi.
            unsafe { std::env::remove_var(self.key) };
        }
    }
}

pub(super) fn environment_test_guard() -> MutexGuard<'static, ()> {
    crate::environment_test_guard()
}
