//! Behavior tests for the git_provenance owner.
//! Test hành vi cho owner git_provenance.

use super::super::*;

#[test]
fn commit_lookup_returns_first_matching_marker_for_exact_package_name() {
    let mut package = mgc_lockfile::Package::new(
        "scope/repo".into(),
        "1.0.0".into(),
        String::new(),
        String::new(),
    );
    package.markers = Some(vec![
        "source:git".into(),
        "git-commit:first".into(),
        "git-commit:second".into(),
    ]);

    assert_eq!(
        git_commit_for_package(&[package], "scope/repo"),
        Some("first".into())
    );
}

#[test]
fn commit_lookup_returns_none_for_a_different_package_name() {
    let package = mgc_lockfile::Package::new(
        "scope/repo".into(),
        "1.0.0".into(),
        String::new(),
        String::new(),
    );

    assert_eq!(git_commit_for_package(&[package], "scope/other"), None);
}
