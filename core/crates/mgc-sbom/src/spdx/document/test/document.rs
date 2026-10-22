#![allow(clippy::unwrap_used)]
//! SPDX document serde tests — Test serde tài liệu SPDX

use super::super::super::*;
use crate::spdx::creation_info::CreationInfo;

#[test]
fn serializes_the_special_id_and_document_describes_fields() {
    let document = SpdxDocument {
        spdx_version: "SPDX-2.3",
        data_license: "CC0-1.0",
        spdx_id: "SPDXRef-DOCUMENT",
        name: "demo".into(),
        document_namespace: "urn:uuid:test".into(),
        creation_info: CreationInfo {
            creators: Vec::new(),
            created: "2026-10-17T00:00:00Z".into(),
        },
        document_describes: vec!["SPDXRef-Package-a".into()],
        packages: Vec::new(),
        relationships: Vec::new(),
    };
    let value = serde_json::to_value(document).unwrap();

    assert_eq!(value["SPDXID"], "SPDXRef-DOCUMENT");
    assert!(value.get("spdxId").is_none());
    assert_eq!(value["documentDescribes"][0], "SPDXRef-Package-a");
    assert_eq!(value["spdxVersion"], "SPDX-2.3");
}
