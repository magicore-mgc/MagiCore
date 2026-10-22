#![allow(clippy::unwrap_used)]
//! V4 graph owner tests — Kiểm thử owner graph v4

use super::super::*;

#[test]
fn rejects_a_non_v4_schema_before_projection() {
    let mut lockfile = mgc_lockfile::canonical::LockfileV4::new("mgc-test");
    lockfile.version = "invalid".to_string();

    let result = generate_with_report(
        &lockfile,
        mgc_lockfile::policy::LockPolicyMode::Warn,
        &[],
        false,
    );

    assert!(matches!(result, Err(GeneratorError::InvalidLockfile(_))));
}
