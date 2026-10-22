//! Focused tests for Cargo dependency-option policy.
//! Test tập trung cho policy option dependency Cargo.

use super::validate_dependency_specs;
use mgc_lockfile::Package;
use std::collections::BTreeMap;

#[test]
fn accepts_a_locked_registry_dependency_with_supported_options() {
    let package = Package {
        name: "serde".to_string(),
        ..Package::default()
    };
    let by_name = BTreeMap::from([("serde", vec![&package])]);
    let manifest: toml::Value = toml::from_str(
        "[dependencies]\nserde = { version = \"1\", features = [\"derive\"], default-features = false }\n",
    )
    .unwrap();

    assert!(validate_dependency_specs(&manifest, &by_name).is_ok());
}

#[test]
fn rejects_path_dependencies_before_lock_membership() {
    let manifest: toml::Value =
        toml::from_str("[dependencies]\nserde = { path = \"../serde\" }\n").unwrap();
    let error = validate_dependency_specs(&manifest, &BTreeMap::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Cargo projection does not support path/git/workspace/alias/registry dependency 'serde'"
    );
}

#[test]
fn rejects_dependencies_absent_from_the_lock() {
    let manifest: toml::Value = toml::from_str("[dependencies]\nserde = \"1\"\n").unwrap();
    let error = validate_dependency_specs(&manifest, &BTreeMap::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Cargo dependency 'serde' is absent from mgc.lock; run `mgc install`"
    );
}
