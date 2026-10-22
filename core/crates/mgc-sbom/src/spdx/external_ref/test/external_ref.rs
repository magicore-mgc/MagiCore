#![allow(clippy::unwrap_used)]
//! External reference serde tests — Test serde external reference

use super::super::super::*;

#[test]
fn serializes_package_manager_purl_reference_fields() {
    let value = serde_json::to_value(SpdxExternalRef {
        reference_category: "PACKAGE-MANAGER",
        reference_type: "purl",
        reference_locator: "pkg:cargo/demo@1.0.0".into(),
    })
    .unwrap();

    assert_eq!(value["referenceCategory"], "PACKAGE-MANAGER");
    assert_eq!(value["referenceType"], "purl");
    assert_eq!(value["referenceLocator"], "pkg:cargo/demo@1.0.0");
}
