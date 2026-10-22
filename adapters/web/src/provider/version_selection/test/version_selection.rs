//! Stable/prerelease selection / chọn stable và prerelease.

use crate::provider::NpmDependencyProvider;
use mgc_types::Version;

#[test]
fn prefers_stable_unless_the_spec_requests_a_prerelease() {
    let versions = [
        Version::parse("4.4.3").unwrap(),
        Version::parse("4.5.0-canary.20260504T180558").unwrap(),
    ];
    assert_eq!(
        NpmDependencyProvider::select_best_version(&versions, "^4.0.0")
            .unwrap()
            .unwrap()
            .to_string(),
        "4.4.3"
    );
    assert_eq!(
        NpmDependencyProvider::select_best_version(&versions, "4.5.0-canary.20260504T180558")
            .unwrap()
            .unwrap()
            .to_string(),
        "4.5.0-canary.20260504T180558"
    );
}
