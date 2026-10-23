//! Live-attestation behavior tests / Test hành vi attestation hiện hữu.
//! Protects the mode Off early return without requiring host identities.

#![allow(clippy::unwrap_used, unsafe_code)]

use super::super::enforce_live_attestation;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct AnchorModeRestore(Option<std::ffi::OsString>);

impl AnchorModeRestore {
    fn off() -> Self {
        let previous = std::env::var_os(crate::attestation::ANCHOR_MODE_ENV);
        // SAFETY: test-only env mutation; callers hold `env_lock` so no other
        // test thread observes a partially written environment.
        // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
        // không thấy môi trường ghi dở.
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe { std::env::set_var(crate::attestation::ANCHOR_MODE_ENV, "off") };
        Self(previous)
    }
}

impl Drop for AnchorModeRestore {
    fn drop(&mut self) {
        match self.0.take() {
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            Some(value) => unsafe { std::env::set_var(crate::attestation::ANCHOR_MODE_ENV, value) },
            // SAFETY: test-only operation on process-local state or FFI with
            // caller-owned arguments valid for the call.
            // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
            // đối số do caller sở hữu, hợp lệ trong lời gọi.
            None => unsafe { std::env::remove_var(crate::attestation::ANCHOR_MODE_ENV) },
        }
    }
}

#[test]
fn off_mode_accepts_project_without_attestation() {
    let _lock = ENV_LOCK.lock().expect("anchor environment lock");
    let _mode = AnchorModeRestore::off();
    let project = tempfile::tempdir().expect("project directory");
    assert!(enforce_live_attestation(project.path(), "web").is_ok());
}
