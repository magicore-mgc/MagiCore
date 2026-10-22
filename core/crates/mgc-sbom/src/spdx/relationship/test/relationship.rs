#![allow(clippy::unwrap_used)]
//! SPDX relationship serde tests — Test serde relationship SPDX

use super::super::super::*;

#[test]
fn serializes_dependency_relationship_fields() {
    let relationship = SpdxRelationship {
        spdx_element_id: "SPDXRef-Package-a".into(),
        relationship_type: "DEPENDS_ON",
        related_spdx_element: "SPDXRef-Package-b".into(),
    };
    let value = serde_json::to_value(relationship).unwrap();

    assert_eq!(value["spdxElementId"], "SPDXRef-Package-a");
    assert_eq!(value["relationshipType"], "DEPENDS_ON");
    assert_eq!(value["relatedSpdxElement"], "SPDXRef-Package-b");
}
