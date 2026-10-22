#![allow(clippy::unwrap_used)]
//! Tool serialization tests — Test tuần tự hóa tool

use super::super::super::*;

#[test]
fn serializes_vendor_name_and_version_without_renaming() {
    let value = serde_json::to_value(Tool {
        vendor: "MagiCore".into(),
        name: "mgc".into(),
        version: "1.0.0".into(),
    })
    .unwrap();

    assert_eq!(value["vendor"], "MagiCore");
    assert_eq!(value["name"], "mgc");
    assert_eq!(value["version"], "1.0.0");
}
