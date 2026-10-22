//! Stable release and range eligibility tests.
//! Test stable release và range đủ điều kiện.

use super::super::super as versions;

#[test]
#[cfg(feature = "web")]
fn scaffold_selects_latest_release_that_passes_the_project_age_gate() {
    use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
    use std::collections::HashMap;

    let now = chrono::Utc::now();
    let old_release =
        (now - chrono::Duration::hours(48)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let fresh_release =
        (now - chrono::Duration::hours(1)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let version_info = |version: &str| VersionInfo {
        version: version.to_string(),
        dependencies: None,
        dev_dependencies: None,
        peer_dependencies: None,
        peer_dependencies_meta: HashMap::new(),
        optional_dependencies: None,
        os: None,
        cpu: None,
        dist: None,
    };
    let metadata = PackageMetadata {
        name: "react-dom".to_string(),
        description: None,
        versions: HashMap::from([
            ("19.3.0".to_string(), version_info("19.3.0")),
            ("19.3.1".to_string(), version_info("19.3.1")),
        ]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([
            ("19.3.0".to_string(), old_release.clone()),
            ("19.3.1".to_string(), fresh_release),
        ]),
    };

    let age_policy = versions::ScaffoldAgePolicy {
        cutoff_hours: 24,
        allow_missing_time: false,
    };
    assert!(
        versions::validate_seed_range_age_eligible("react-dom", &metadata, age_policy, "^19.3.0",)
            .is_ok()
    );
    assert!(
        versions::validate_seed_range_age_eligible("react-dom", &metadata, age_policy, "^19.3.1",)
            .is_err()
    );

    let selected = versions::select_latest_eligible_version(
        "react-dom",
        &metadata,
        Some(versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap();

    assert_eq!(selected, "^19.3.0");

    let react_metadata = PackageMetadata {
        name: "react".to_string(),
        description: None,
        versions: HashMap::from([("19.3.1".to_string(), version_info("19.3.1"))]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([("19.3.1".to_string(), old_release)]),
    };
    let selected_react = versions::select_latest_eligible_version(
        "react",
        &react_metadata,
        Some(versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap();

    assert_eq!(selected_react, "^19.3.1");
}

#[test]
#[cfg(feature = "web")]
fn scaffold_age_gate_fails_closed_when_every_release_is_too_new() {
    use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
    use std::collections::HashMap;

    let fresh_release = (chrono::Utc::now() - chrono::Duration::hours(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let metadata = PackageMetadata {
        name: "react-dom".to_string(),
        description: None,
        versions: HashMap::from([(
            "19.3.1".to_string(),
            VersionInfo {
                version: "19.3.1".to_string(),
                dependencies: None,
                dev_dependencies: None,
                peer_dependencies: None,
                peer_dependencies_meta: HashMap::new(),
                optional_dependencies: None,
                os: None,
                cpu: None,
                dist: None,
            },
        )]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([("19.3.1".to_string(), fresh_release)]),
    };

    assert!(
        versions::select_latest_eligible_version(
            "react-dom",
            &metadata,
            Some(versions::ScaffoldAgePolicy {
                cutoff_hours: 24,
                allow_missing_time: false,
            }),
        )
        .is_err()
    );
}
