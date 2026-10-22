//! Covers package defaults used by legacy lockfile documents.
//! Kiểm tra mặc định package dùng khi đọc lockfile cũ.

use super::super::Package;
use crate::ecosystem_tag::EcosystemTag;

#[test]
fn defaults_legacy_fields_and_omits_the_other_ecosystem() {
    let package: Package = serde_json::from_value(serde_json::json!({
        "name": "sample",
        "version": "1.0.0",
        "resolved": "https://example.invalid/sample.tgz",
        "integrity": "blake3-test"
    }))
    .expect("test operation must succeed");

    assert_eq!(package.ecosystem, EcosystemTag::Other);
    assert!(package.dependencies.is_empty());
    assert!(package.owner_core.is_none());
    assert!(package.registry.is_none());
    let serialized = serde_json::to_value(package).expect("test operation must succeed");
    assert!(serialized.get("ecosystem").is_none());
}
