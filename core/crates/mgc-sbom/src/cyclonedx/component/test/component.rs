#![allow(clippy::unwrap_used)]
//! Component serialization tests — Test tuần tự hóa component

use super::super::super::*;
use crate::cyclonedx::ComponentType;

#[test]
fn preserves_special_keys_and_omits_absent_optional_fields() {
    let value = serde_json::to_value(Component {
        component_type: ComponentType::Library,
        bom_ref: "pkg:cargo/demo@1.0.0".into(),
        name: "demo".into(),
        version: "1.0.0".into(),
        purl: None,
        hashes: None,
        licenses: None,
    })
    .unwrap();

    assert_eq!(value["type"], "library");
    assert_eq!(value["bom-ref"], "pkg:cargo/demo@1.0.0");
    assert!(value.get("componentType").is_none());
    assert!(value.get("bomRef").is_none());
    assert!(value.get("purl").is_none());
    assert!(value.get("hashes").is_none());
    assert!(value.get("licenses").is_none());
}
