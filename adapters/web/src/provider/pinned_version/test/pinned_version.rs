//! Explicit pin age-policy behavior / policy tuổi với version pin.

use crate::native::npm_registry::PackageMetadata;
use crate::provider::{AgePolicy, check_pinned_version};
use mgc_types::PackageName;
use std::collections::HashMap;

#[test]
fn rejects_young_pins_and_only_allows_missing_time_by_explicit_policy() {
    let package = PackageName::new("react").unwrap();
    let metadata = PackageMetadata {
        name: "react".into(),
        description: None,
        versions: HashMap::new(),
        dist_tags: HashMap::new(),
        time: HashMap::from([
            ("old".into(), "2020-01-01T00:00:00Z".into()),
            ("young".into(), "2100-01-01T00:00:00Z".into()),
        ]),
    };
    let strict = AgePolicy {
        cutoff_hours: 1,
        allow_missing_time: false,
    };
    assert!(check_pinned_version(&package, &metadata, "old", Some(strict)).is_ok());
    assert!(check_pinned_version(&package, &metadata, "young", Some(strict)).is_err());
    assert!(check_pinned_version(&package, &metadata, "missing", Some(strict)).is_err());
    assert!(
        check_pinned_version(
            &package,
            &metadata,
            "missing",
            Some(AgePolicy {
                allow_missing_time: true,
                ..strict
            })
        )
        .is_ok()
    );
}
