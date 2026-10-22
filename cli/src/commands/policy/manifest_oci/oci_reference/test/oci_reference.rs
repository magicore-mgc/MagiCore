//! Verify OCI host, repository, tag and default handling.
//! Xác minh xử lý host, repository, tag và mặc định OCI.

#![allow(clippy::unwrap_used)]

use super::parse_oci_reference;

#[test]
fn parses_registry_repository_and_default_tag() {
    assert_eq!(
        parse_oci_reference("oci://registry.example.com/magicore/policy:exceptions"),
        Some((
            "https://registry.example.com".to_string(),
            "magicore/policy".to_string(),
            "exceptions".to_string()
        ))
    );
    assert_eq!(
        parse_oci_reference("oci://127.0.0.1:5000/p:1"),
        Some((
            "http://127.0.0.1:5000".to_string(),
            "p".to_string(),
            "1".to_string()
        ))
    );
    assert_eq!(
        parse_oci_reference("oci://registry.example.com/p"),
        Some((
            "https://registry.example.com".to_string(),
            "p".to_string(),
            "latest".to_string()
        ))
    );
    assert!(parse_oci_reference("https://registry.example.com/p").is_none());
    assert!(parse_oci_reference("oci://onlyhost").is_none());
}
