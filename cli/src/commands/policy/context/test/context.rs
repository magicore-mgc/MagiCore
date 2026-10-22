//! Verify a missing lock cannot activate a policy profile.
//! Xác nhận thiếu lock không thể kích hoạt profile policy.

#![allow(clippy::unwrap_used)]

use super::active_profile;

#[test]
fn missing_lock_cannot_activate_a_policy_profile() {
    let directory = tempfile::tempdir().unwrap();
    let context = mgc_exec::policy::PolicyContext {
        framework: "vite".to_string(),
        version: "6.0.0".to_string(),
        os: std::env::consts::OS.to_string(),
        operation: "run".to_string(),
    };

    assert!(active_profile(directory.path(), &context).is_none());
}
