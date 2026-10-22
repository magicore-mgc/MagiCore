//! Release-age policy contract / contract policy tuổi release.

#[test]
fn age_gate_parses_npm_time_and_filters() {
    use crate::native::npm_registry::{PackageMetadata, VersionInfo};
    use crate::provider::{AgePolicy, eligible_versions, parse_npm_time};
    use mgc_types::PackageName;
    // 2020-01-01T00:00:00Z == 1577836800 (known anchor).
    assert_eq!(parse_npm_time("2020-01-01T00:00:00.000Z"), Some(1577836800));
    assert_eq!(parse_npm_time("2020-01-01T00:00:00Z"), Some(1577836800));
    assert_eq!(parse_npm_time("garbage"), None);
    assert_eq!(parse_npm_time("2020-13-01T00:00:00Z"), None);
    fn version_info(version: &str) -> VersionInfo {
        VersionInfo {
            version: version.to_string(),
            dependencies: None,
            dev_dependencies: None,
            peer_dependencies: None,
            peer_dependencies_meta: Default::default(),
            optional_dependencies: None,
            os: None,
            cpu: None,
            dist: None,
        }
    }
    let mut versions = std::collections::HashMap::new();
    versions.insert("1.0.0".to_string(), version_info("1.0.0"));
    versions.insert("2.0.0".to_string(), version_info("2.0.0"));
    let mut time = std::collections::HashMap::new();
    time.insert("1.0.0".to_string(), "2020-01-01T00:00:00.000Z".to_string());
    time.insert("2.0.0".to_string(), "2100-01-01T00:00:00.000Z".to_string());
    let meta = PackageMetadata {
        name: "x".to_string(),
        description: None,
        versions,
        dist_tags: std::collections::HashMap::new(),
        time,
    };
    let package = PackageName::new("x").unwrap();
    let policy = AgePolicy {
        cutoff_hours: 10000,
        allow_missing_time: false,
    };
    // No policy: historical behavior (everything).
    let kept = eligible_versions(&package, &meta, None).unwrap();
    assert_eq!(kept.len(), 2);
    // Policy: old version survives, future version excluded.
    let kept = eligible_versions(&package, &meta, Some(policy)).unwrap();
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].to_string(), "1.0.0");
    // Missing timestamps are REJECTED by default (fail-closed)...
    let mut meta_unstamped = meta.clone();
    meta_unstamped.time.clear();
    let err = eligible_versions(&package, &meta_unstamped, Some(policy)).unwrap_err();
    assert!(err.contains("unstamped"));
    // ...unless the explicit private-registry escape hatch is set.
    let policy = AgePolicy {
        allow_missing_time: true,
        ..policy
    };
    let kept = eligible_versions(&package, &meta_unstamped, Some(policy)).unwrap();
    assert_eq!(kept.len(), 2);
}
