#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn rejects_unknown_core_before_project_or_keyring_io() {
    let project = tempfile::tempdir().expect("project directory");
    let keyring = project.path().join("missing/keyring.json");
    let identities = project.path().join("identities");
    let error = attest(
        project.path(),
        "not-a-core",
        &keyring,
        &identities,
        None,
        false,
        false,
    )
    .expect_err("unknown core must be rejected");
    assert!(error.to_string().contains("cannot attest unknown core"));
    assert!(!keyring.exists());
}
