//! Tests registry and Git pin identity and requirement projection.
//! Kiểm tra identity pin registry/Git và projection requirement.

use super::super::pin_range_for;
use mgc_resolver::protocols::swift::{SwiftDep, SwiftResolvedPin};

#[test]
fn registry_version_pin_is_exact_and_case_insensitive() {
    let dep = SwiftDep::Registry {
        identity: "Scope.Library".to_string(),
        requirement: "from:1.0.0".to_string(),
    };
    let pin = SwiftResolvedPin {
        identity: "scope.library".to_string(),
        version: Some("1.4.2".to_string()),
        location: Some("registry+https://packages.example.test".to_string()),
        ..SwiftResolvedPin::default()
    };

    assert_eq!(pin_range_for(&dep, &[pin]), Some("exact:1.4.2".to_string()));
}

#[test]
fn git_pin_matches_repository_location_and_projects_version_as_tag() {
    let dep = SwiftDep::Git {
        url: "https://github.com/Org/Library.git".to_string(),
        requirement: "branch:main".to_string(),
    };
    let pin = SwiftResolvedPin {
        identity: "other-name".to_string(),
        version: Some("2.0.1".to_string()),
        location: Some("https://github.com/Org/Library.git".to_string()),
        ..SwiftResolvedPin::default()
    };

    assert_eq!(pin_range_for(&dep, &[pin]), Some("tag:2.0.1".to_string()));
}
