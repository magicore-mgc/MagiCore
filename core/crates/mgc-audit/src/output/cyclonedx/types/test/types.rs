//! Verify schema field names, CycloneDX 1.5 tools shape and empty sections.
//! Kiểm tra tên field, hình tools CycloneDX 1.5 và section rỗng.

use super::*;

#[test]
fn bom_serializes_cyclonedx_names_and_omits_empty_sections() {
    let bom = CycloneDxBom {
        bom_format: CYCLONEDX_BOM_FORMAT.to_string(),
        spec_version: CYCLONEDX_SPEC_VERSION.to_string(),
        serial_number: "urn:uuid:serial".to_string(),
        version: 1,
        metadata: CycloneDxMetadata {
            timestamp: "2026-10-19T00:00:00Z".to_string(),
            tools: CycloneDxTools {
                components: vec![CycloneDxTool {
                    tool_type: "application",
                    vendor: "MagiCore",
                    name: "mgc-audit",
                    version: "1.1.0".to_string(),
                    properties: Vec::new(),
                }],
            },
        },
        components: Vec::new(),
        vulnerabilities: Vec::new(),
    };

    let value = serde_json::to_value(bom).unwrap();

    assert_eq!(value["bomFormat"], "CycloneDX");
    assert_eq!(value["specVersion"], "1.5");
    assert!(value["metadata"]["tools"].is_object());
    assert!(value["components"].is_null());
    assert!(value["vulnerabilities"].is_null());
}
