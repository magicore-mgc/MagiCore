//! Shared fixtures and process-environment guards for lifecycle unit tests.
//! Fixture chung và guard biến môi trường process cho test lifecycle.

use std::ffi::OsString;
use std::sync::MutexGuard;

/// Absolute interpreter path for test scripts.
/// Đường dẫn tuyệt đối của interpreter dùng trong script test.
///
/// SEC-02 moved bare `python3` out of the install scope, and a project-local
/// executable is allowed only under an absolute path. Tests therefore address
/// the host interpreter explicitly, exactly like a real project would.
/// (SEC-02 đưa `python3` trần ra khỏi scope install; executable cục bộ project
/// chỉ được dùng qua đường dẫn tuyệt đối. Vì vậy test gọi interpreter bằng
/// đường dẫn tuyệt đối, giống project thật.)
pub(super) fn install_node_modules_bin(package: &std::path::Path, name: &str, target: &str) {
    let bin_dir = package.join("node_modules").join(".bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = bin_dir.join(name);
        if !link.exists() {
            symlink(target, link).unwrap();
        }
    }
    #[cfg(not(unix))]
    {
        std::fs::write(
            bin_dir.join(format!("{name}.cmd")),
            format!("@\"{target}\" %*\r\n"),
        )
        .unwrap();
    }
}

pub(super) fn python3_path() -> String {
    if std::path::Path::new("/usr/bin/python3").exists() {
        "/usr/bin/python3".to_string()
    } else if std::path::Path::new("/opt/homebrew/bin/python3").exists() {
        "/opt/homebrew/bin/python3".to_string()
    } else {
        "/usr/local/bin/python3".to_string()
    }
}

/// Write a package manifest whose scripts run through an interpreter fixture.
///
/// SEC-02 keeps language runtimes out of the install scope, so every fixture
/// interpreter is first installed as a project-local binary; hooks then spawn
/// it by relative path, which is exactly what a real npm package does with its
/// vendored binaries.
/// (SEC-02 giữ runtime ngôn ngữ ra khỏi scope install, nên mọi interpreter
/// fixture được cài trước thành binary cục bộ của project; hook gọi nó bằng
/// đường dẫn tương đối, đúng như package npm thật gọi binary vendored.)
pub(super) fn write_package_script(package: &std::path::Path, script: &str) {
    install_node_modules_bin(package, "python3", &python3_path());
    let script = script.replace("@PYTHON@", "./node_modules/.bin/python3");
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
        let guard = crate::environment_test_guard();
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
