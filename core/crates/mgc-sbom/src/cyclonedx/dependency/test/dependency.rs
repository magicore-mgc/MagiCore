#![allow(clippy::unwrap_used)]
//! Dependency serialization tests — Test tuần tự hóa dependency

use super::super::super::*;

#[test]
fn serializes_special_ref_and_omits_absent_edges() {
    let value = serde_json::to_value(Dependency {
        dependency_ref: "pkg:cargo/demo@1.0.0".into(),
        depends_on: None,
    })
    .unwrap();

    assert_eq!(value["ref"], "pkg:cargo/demo@1.0.0");
    assert!(value.get("dependencyRef").is_none());
    assert!(value.get("dependsOn").is_none());
}
