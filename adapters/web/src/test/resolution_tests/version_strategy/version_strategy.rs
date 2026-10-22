// version_strategy behavior tests — test behavior thuộc owner version_strategy.
use super::*;

#[test]
fn add_range_does_not_masquerade_as_resolved_version() {
    assert!(
        super::exact_version_from_range(&mgc_types::VersionRange::parse("^1.2.3").unwrap())
            .is_none()
    );
    assert!(
        super::exact_version_from_range(&mgc_types::VersionRange::parse(">=1.2.3").unwrap())
            .is_none()
    );
    assert_eq!(
        super::exact_version_from_range(&mgc_types::VersionRange::parse("1.2.3").unwrap())
            .unwrap()
            .to_string(),
        "1.2.3"
    );
}

#[test]
fn test_preferred_saved_range_preserves_strategy() {
    assert_eq!(
        WebAdapter::preferred_saved_range(&VersionRange::parse("^4.0.0").unwrap(), "5.1.2")
            .unwrap()
            .to_string(),
        "^5.1.2"
    );
    assert_eq!(
        WebAdapter::preferred_saved_range(&VersionRange::parse("~4.0.0").unwrap(), "5.1.2")
            .unwrap()
            .to_string(),
        "~5.1.2"
    );
    assert_eq!(
        WebAdapter::preferred_saved_range(&VersionRange::parse("*").unwrap(), "5.1.2")
            .unwrap()
            .to_string(),
        "^5.1.2"
    );
    assert_eq!(
        WebAdapter::preferred_saved_range(&VersionRange::parse("4.0.0").unwrap(), "5.1.2")
            .unwrap()
            .to_string(),
        "5.1.2"
    );
}

#[test]
fn test_preferred_registry_version_prefers_stable_over_prerelease() {
    let metadata = native::npm_registry::PackageMetadata {
        name: "demo".into(),
        description: None,
        versions: std::collections::HashMap::from([
            (
                "4.4.3".into(),
                native::npm_registry::VersionInfo {
                    version: "4.4.3".into(),
                    dependencies: None,
                    dev_dependencies: None,
                    optional_dependencies: None,
                    peer_dependencies: None,
                    peer_dependencies_meta: Default::default(),
                    os: None,
                    cpu: None,
                    dist: None,
                },
            ),
            (
                "4.5.0-canary.20260504T180558".into(),
                native::npm_registry::VersionInfo {
                    version: "4.5.0-canary.20260504T180558".into(),
                    dependencies: None,
                    dev_dependencies: None,
                    optional_dependencies: None,
                    peer_dependencies: None,
                    peer_dependencies_meta: Default::default(),
                    os: None,
                    cpu: None,
                    dist: None,
                },
            ),
        ]),
        dist_tags: std::collections::HashMap::from([(
            "latest".into(),
            "4.5.0-canary.20260504T180558".into(),
        )]),
        time: std::collections::HashMap::new(),
    };

    assert_eq!(
        preferred_registry_version(&metadata).as_deref(),
        Some("4.4.3")
    );
}
