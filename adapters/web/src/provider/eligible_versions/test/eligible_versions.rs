//! Age-filtered candidate failure contract / contract loại hết candidate theo age gate.

use crate::native::npm_registry::PackageMetadata;
use crate::provider::{AgePolicy, eligible_versions};
use mgc_types::PackageName;
use std::collections::HashMap;

#[test]
fn rejects_a_nonempty_registry_when_every_candidate_is_too_young() {
    let metadata = PackageMetadata {
        name: "react".into(),
        description: None,
        versions: HashMap::from([(
            "18.3.0".into(),
            crate::native::npm_registry::VersionInfo {
                version: "18.3.0".into(),
                dependencies: None,
                dev_dependencies: None,
                peer_dependencies: None,
                peer_dependencies_meta: Default::default(),
                optional_dependencies: None,
                os: None,
                cpu: None,
                dist: None,
            },
        )]),
        dist_tags: HashMap::new(),
        time: HashMap::from([("18.3.0".into(), "2100-01-01T00:00:00Z".into())]),
    };
    let package = PackageName::new("react").unwrap();
    let error = eligible_versions(
        &package,
        &metadata,
        Some(AgePolicy {
            cutoff_hours: 1,
            allow_missing_time: false,
        }),
    )
    .unwrap_err();
    assert!(error.contains("all 1 version(s) of react excluded"));
}
