//! Focused tests for Cargo manifest root matching.
//! Test tập trung cho đối chiếu root manifest Cargo.

use super::resolve_manifest_roots;
use mgc_lockfile::Package;
use std::collections::BTreeMap;

#[test]
fn resolves_matching_manifest_and_lock_roots() {
    let package = Package {
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        ..Package::default()
    };
    let id = "serde@1.0.0".to_string();
    let by_id = BTreeMap::from([(id.clone(), &package)]);
    let by_name = BTreeMap::from([("serde", vec![&package])]);
    let manifest: toml::Value = toml::from_str("[dependencies]\nserde = \"1\"\n").unwrap();

    let roots = resolve_manifest_roots(&manifest, &[id], &by_id, &by_name).unwrap();
    assert_eq!(roots.len(), 1);
    assert!(roots[0].contains("serde"));
}

#[test]
fn rejects_a_manifest_root_missing_from_the_lock_roots() {
    let package = Package {
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        ..Package::default()
    };
    let by_id = BTreeMap::from([("serde@1.0.0".to_string(), &package)]);
    let by_name = BTreeMap::from([("serde", vec![&package])]);
    let manifest: toml::Value = toml::from_str("[dependencies]\nserde = \"1\"\n").unwrap();

    let error = resolve_manifest_roots(&manifest, &[], &by_id, &by_name).unwrap_err();
    assert_eq!(
        error.to_string(),
        "mgc.lock direct roots do not match Cargo.toml [dependencies]; run `mgc install`"
    );
}
