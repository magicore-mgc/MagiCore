#![allow(clippy::unwrap_used)]
//! BOM serialization tests — Test tuần tự hóa BOM

use super::super::super::*;

#[test]
fn defaults_keep_the_current_schema_identity_and_omit_absent_dependencies() {
    let bom = Bom::default();
    let value = serde_json::to_value(&bom).unwrap();

    assert_eq!(bom.bom_format, "CycloneDX");
    assert_eq!(bom.spec_version, "1.5");
    assert_eq!(bom.version, 1);
    assert!(bom.serial_number.starts_with("urn:uuid:"));
    assert_eq!(bom.metadata.tools[0].vendor, "MagiCore");
    assert_eq!(bom.metadata.tools[0].name, "mgc");
    assert!(value.get("bomFormat").is_some());
    assert!(value.get("specVersion").is_some());
    assert!(value.get("dependencies").is_none());
}
