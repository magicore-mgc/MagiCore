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
fn test_preferred_saved_range_moves_the_range_not_just_the_prefix() {
    // P0 regression: `update` called `prepare_add` with NO range and got the
    // project's existing range back verbatim, so `typescript: ~6.0.2` stayed
    // `~6.0.2` while "Updated N package(s)" was printed. Retargeting must move
    // the version while keeping the declared style.
    // (Hồi quy P0: `update` gọi `prepare_add` không kèm range và nhận lại
    // nguyên range cũ, nên `typescript: ~6.0.2` vẫn `~6.0.2` dù in "Updated N".
    // Đổi đích phải nâng version và giữ kiểu range đã khai.)
    for (current, latest) in [("~6.0.2", "6.0.3"), ("^3.0.0", "3.1.0"), ("6.0.2", "6.1.0")] {
        let declared = VersionRange::parse(current).unwrap();
        let retargeted = WebAdapter::preferred_saved_range(&declared, latest).unwrap();
        assert_ne!(
            retargeted.as_str(),
            current,
            "{current} must move to {latest}, not stay put"
        );
        assert!(
            retargeted.matches(&mgc_types::Version::parse(latest).unwrap()),
            "'{}' must accept the resolved version {latest}",
            retargeted.as_str()
        );
    }
}

#[test]
fn test_preferred_saved_range_keeps_the_tilde_style_on_a_major_jump() {
    // A `~` pin cannot express a MAJOR jump; retargeting still keeps the `~`
    // style so the recorded range is what the user asked for, and the caller
    // separately reports that the jump was not expressible.
    // (Pin `~` không biểu diễn được nhảy MAJOR; đổi đích vẫn giữ kiểu `~` để
    // range ghi lại đúng ý người dùng, còn bước gọi tự báo là không biểu diễn được.)
    let declared = VersionRange::parse("~6.0.2").unwrap();
    let saved = WebAdapter::preferred_saved_range(&declared, "7.0.2").unwrap();
    assert_eq!(saved.as_str(), "~7.0.2");
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
