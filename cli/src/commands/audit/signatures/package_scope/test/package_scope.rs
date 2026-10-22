//! Core namespace behavior for OCI signature audit.
//! Hành vi namespace core khi audit chữ ký OCI.

use super::package_scope::effective_signature_package;

#[test]
fn signature_audit_core_scopes_oci_repository() {
    assert_eq!(
        effective_signature_package(Some("ai"), "oci", "models/weights").unwrap(),
        "ai/models/weights"
    );
    assert_eq!(
        effective_signature_package(Some("clo"), "oci", "artifacts/app").unwrap(),
        "cloud/artifacts/app"
    );
    assert_eq!(
        effective_signature_package(Some("ai"), "npm", "@acme/widgets").unwrap(),
        "@acme/widgets"
    );
    assert!(effective_signature_package(Some("unknown"), "oci", "models/weights").is_err());
}
