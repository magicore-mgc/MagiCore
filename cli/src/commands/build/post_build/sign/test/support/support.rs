//! One HOME-redirecting sandbox shared by every signing test.
//! Một sandbox đổi HOME dùng chung cho mọi test ký.
//!
//! `HOME` and `MGC_SIGNING_KEYRING` are process-wide, so the two test owners
//! beside each other must synchronise on ONE lock; separate per-file locks let
//! them interleave and read each other's keyrings. The lock and the redirected
//! home therefore live here, not in each test file.
//! `HOME` và `MGC_SIGNING_KEYRING` là biến toàn process, nên hai owner test cạnh
//! nhau phải đồng bộ trên MỘT lock; lock riêng từng file cho phép chúng xen kẽ và
//! đọc keyring của nhau. Vì vậy lock và home đã đổi nằm ở đây, không ở từng file.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Redirected-home sandbox; restores the environment when dropped.
/// Sandbox home đã đổi; hoàn nguyên môi trường khi drop.
pub(crate) struct Sandbox {
    root: tempfile::TempDir,
    previous_home: Option<std::ffi::OsString>,
    previous_override: Option<std::ffi::OsString>,
    _guard: MutexGuard<'static, ()>,
}

/// The single lock every signing test that touches the environment holds.
/// Lock duy nhất mà mọi test ký chạm môi trường đều giữ.
fn env_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Sandbox {
    pub(crate) fn new() -> Self {
        let guard = env_lock();
        let root = tempfile::tempdir().expect("test operation must succeed");
        let home = root.path().join("home");
        std::fs::create_dir_all(&home).expect("test operation must succeed");
        let previous_home = std::env::var_os("HOME");
        let previous_override = std::env::var_os("MGC_SIGNING_KEYRING");
        set_env("HOME", home.as_os_str());
        remove_env("MGC_SIGNING_KEYRING");
        Self {
            root,
            previous_home,
            previous_override,
            _guard: guard,
        }
    }

    /// Sandbox root (data dir and artifacts live here).
    /// Root sandbox (data dir và artifact nằm ở đây).
    pub(crate) fn root(&self) -> &std::path::Path {
        self.root.path()
    }

    /// The redirected home directory.
    /// Thư mục home đã đổi.
    pub(crate) fn home(&self) -> PathBuf {
        self.root.path().join("home")
    }

    /// Path `mgc trust` writes.
    /// Đường dẫn `mgc trust` ghi.
    pub(crate) fn trust_keyring(&self) -> PathBuf {
        self.home().join(".magicore/keys/keyring.json")
    }

    /// MagiCore data directory passed to `sign_artifact`.
    /// Data dir MagiCore truyền cho `sign_artifact`.
    pub(crate) fn data_dir(&self) -> PathBuf {
        self.root.path().join("data")
    }

    /// Legacy single-seed keyring path inside the data directory.
    /// Đường dẫn keyring một-seed kiểu cũ trong data dir.
    pub(crate) fn legacy_keyring(&self) -> PathBuf {
        self.data_dir().join("keyring.json")
    }

    /// Point `MGC_SIGNING_KEYRING` at an explicit file.
    /// Trỏ `MGC_SIGNING_KEYRING` tới file tường minh.
    pub(crate) fn pin_keyring_env(&self, path: &std::path::Path) {
        set_env("MGC_SIGNING_KEYRING", path.as_os_str());
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        match &self.previous_home {
            Some(value) => set_env("HOME", value),
            None => remove_env("HOME"),
        }
        match &self.previous_override {
            Some(value) => set_env("MGC_SIGNING_KEYRING", value),
            None => remove_env("MGC_SIGNING_KEYRING"),
        }
    }
}

// Production code never mutates the environment; only these sandboxes do, and
// always while holding `env_lock()`.
// (Code production không bao giờ sửa môi trường; chỉ các sandbox này, và luôn
// trong lúc giữ `env_lock()`.)
#[allow(unsafe_code)]
fn set_env(key: &str, value: &std::ffi::OsStr) {
    // SAFETY: the caller holds `env_lock()`, so no other test thread reads or
    // writes the environment concurrently.
    // (SAFETY: caller giữ `env_lock()` nên không thread test nào đọc/ghi môi
    // trường cùng lúc.)
    unsafe { std::env::set_var(key, value) }
}

#[allow(unsafe_code)]
fn remove_env(key: &str) {
    // SAFETY: see `set_env`.
    // (SAFETY: xem `set_env`.)
    unsafe { std::env::remove_var(key) }
}
