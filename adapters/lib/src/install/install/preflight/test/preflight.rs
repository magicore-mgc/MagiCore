//! Tests for the native Lib install lock preflight.
//! Test preflight lock của install native Lib.

use super::super::validate_native_install_inputs;
use crate::language::LibLanguage;
use mgc_types::ResolvedGraph;

#[test]
fn malformed_existing_lock_is_rejected_before_install_mutation() {
    let project = tempfile::tempdir().unwrap();
    let lock_path = project.path().join("mgc.lock");
    std::fs::write(&lock_path, "not = [valid toml").unwrap();

    let error = validate_native_install_inputs(
        LibLanguage::Python,
        &ResolvedGraph::empty(),
        project.path(),
        &[],
    )
    .unwrap_err();

    assert!(matches!(error, mgc_types::MgError::Other(_)));
    assert_eq!(
        std::fs::read_to_string(lock_path).unwrap(),
        "not = [valid toml"
    );
}
