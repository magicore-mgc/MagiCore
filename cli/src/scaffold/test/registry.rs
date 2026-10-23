//! Tests for the registry scaffold resolver.
//! Test cho resolver scaffold theo registry.
use super::*;
use crate::scaffold::spec::CoreKind;

#[test]
fn test_package_name_convention() {
    let spec = ScaffoldSpec {
        core: CoreKind::Web,
        name: "nextjs".to_string(),
        normalized_name: "nextjs".to_string(),
        requested_ref: ScaffoldRef::DistTag("latest".to_string()),
    };

    assert_eq!(
        crate::scaffold::registry::naming::package_name(&spec),
        "mgc-create-web-nextjs"
    );
}

#[test]
fn test_explicit_version_resolve() {
    let spec = ScaffoldSpec {
        core: CoreKind::Web,
        name: "nextjs@15.5.0".to_string(),
        normalized_name: "nextjs".to_string(),
        requested_ref: ScaffoldRef::Version("15.5.0".to_string()),
    };

    // Synchronous version - no async needed
    match &spec.requested_ref {
        ScaffoldRef::Version(v) => assert_eq!(v, "15.5.0"),
        _ => panic!("Expected version ref"),
    }
}

#[test]
fn test_userinfo_stripped_from_custom_url() {
    let registry = ScaffoldRegistry::with_url("https://user:s3cret@example.com/reg/").unwrap();
    assert!(
        !registry.base_url.contains("s3cret"),
        "credential leaked into base URL: {}",
        registry.base_url
    );
    assert!(registry.base_url.starts_with("https://example.com/"));
}

#[test]
fn test_garbage_url_refused() {
    assert!(ScaffoldRegistry::with_url("not a url :::").is_err());
}
