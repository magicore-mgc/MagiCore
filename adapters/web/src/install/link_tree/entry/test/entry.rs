//! Verify package entry replaces an old target after creating its parent.
//! Kiểm tra package entry thay target cũ sau khi tạo parent.

#![allow(unsafe_code)]

use std::ffi::OsString;
use std::sync::MutexGuard;

use super::super::implementation::link_package_tree;

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

fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

#[test]
fn package_entry_replaces_stale_target_after_parent_setup() {
    let env = EnvGuard::new("MAGICORE_WEB_STRICT_TREE_MODE");
    env.set(Some("hardlink"));

    let temp = tempdir_real().expect("canonical tempdir");
    let source = temp.path().join("source");
    std::fs::create_dir_all(&source).expect("source directory");
    std::fs::write(source.join("package.json"), b"{\"name\":\"demo\"}").expect("source manifest");

    let target = temp.path().join("nested").join("target");
    std::fs::create_dir_all(&target).expect("stale target");
    std::fs::write(target.join("stale.txt"), b"stale").expect("stale marker");

    link_package_tree(&source, &target).expect("replace stale package target");

    assert!(target.join("package.json").is_file());
    assert!(!target.join("stale.txt").exists());
}
