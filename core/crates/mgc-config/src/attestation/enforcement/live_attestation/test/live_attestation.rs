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
        unsafe { std::env::set_var(crate::attestation::ANCHOR_MODE_ENV, "off") };
        Self(previous)
    }
}

impl Drop for AnchorModeRestore {
    fn drop(&mut self) {
        match self.0.take() {
            Some(value) => unsafe { std::env::set_var(crate::attestation::ANCHOR_MODE_ENV, value) },
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
