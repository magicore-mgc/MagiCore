#![allow(clippy::unwrap_used)]
//! SPDX package serde tests — Test serde package SPDX

use super::super::super::*;

#[test]
fn omits_empty_optional_values_and_uses_the_special_id_key() {
    let package = SpdxPackage {
        spdx_id: "SPDXRef-Package-a".into(),
        name: "demo".into(),
        version_info: None,
        download_location: "NOASSERTION",
        files_analyzed: false,
        license_concluded: "NOASSERTION",
        license_declared: "NOASSERTION",
        copyright_text: "NOASSERTION",
        checksums: Vec::new(),
        external_refs: Vec::new(),
    };
    let value = serde_json::to_value(package).unwrap();

    assert_eq!(value["SPDXID"], "SPDXRef-Package-a");
    assert!(value.get("spdxId").is_none());
    assert!(value.get("versionInfo").is_none());
    assert!(value.get("checksums").is_none());
    assert!(value.get("externalRefs").is_none());
}
