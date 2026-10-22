//! Shared fixtures and process-environment guards for lifecycle unit tests.
//! Fixture chung và guard biến môi trường process cho test lifecycle.

use std::ffi::OsString;
use std::sync::MutexGuard;

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
        let guard = crate::environment_test_lock()
            .lock()
            .expect("lifecycle environment lock poisoned");
        let previous = std::env::var_os(key);
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
            unsafe { std::env::set_var(self.key, value) };
        } else {
            unsafe { std::env::remove_var(self.key) };
        }
    }
}

pub(super) fn environment_test_guard() -> MutexGuard<'static, ()> {
    crate::environment_test_lock()
        .lock()
        .expect("lifecycle environment lock poisoned")
}
