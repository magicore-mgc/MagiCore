//! Dist-tag age eligibility tests.
//! Test dist-tag theo age eligibility.

use super::super::super as versions;

#[test]
#[cfg(feature = "web")]
fn selected_scaffold_dist_tag_obeys_the_release_age_gate() {
    use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
    use std::collections::HashMap;

    let old_release = (chrono::Utc::now() - chrono::Duration::hours(48))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let version_info = VersionInfo {
        version: "15.2.0".to_string(),
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
        name: "next".to_string(),
        description: None,
        versions: HashMap::from([("15.2.0".to_string(), version_info)]),
        dist_tags: HashMap::from([("beta".to_string(), "15.2.0".to_string())]),
        time: HashMap::from([("15.2.0".to_string(), old_release)]),
    };
    let selected = versions::selected_dist_tag_version(
        "next",
        "beta",
        &metadata,
        Some(versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .expect("a dist tag pointing to an eligible release should resolve");

    assert_eq!(selected, "15.2.0");

    let fresh_release = (chrono::Utc::now() - chrono::Duration::hours(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut fresh_metadata = metadata;
    fresh_metadata
        .time
        .insert("15.2.0".to_string(), fresh_release);
    assert!(
        versions::selected_dist_tag_version(
            "next",
            "beta",
            &fresh_metadata,
            Some(versions::ScaffoldAgePolicy {
                cutoff_hours: 24,
                allow_missing_time: false,
            }),
        )
        .is_err()
    );
}
