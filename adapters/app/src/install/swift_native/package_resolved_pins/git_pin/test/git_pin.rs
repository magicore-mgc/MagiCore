//! Behavior tests for the git_pin owner.
//! Test hành vi cho owner git_pin.

use super::super::*;
use mgc_types::Version;

#[test]
fn git_pin_keeps_repository_tail_revision_and_location() {
    let version = Version::parse("2.0.0").unwrap();

    let pin = build_git_pin("github.com/Owner/Repo", &version, Some("abc123".into()));

    assert_eq!(pin.identity, "repo");
    assert_eq!(pin.version.as_deref(), Some("2.0.0"));
    assert_eq!(pin.revision.as_deref(), Some("abc123"));
    assert_eq!(pin.branch, None);
    assert_eq!(
        pin.location.as_deref(),
        Some("https://github.com/Owner/Repo.git")
    );
}

#[test]
fn git_pin_keeps_missing_revision_absent() {
    let version = Version::parse("2.0.0").unwrap();

    let pin = build_git_pin("github.com/Owner/Repo", &version, None);

    assert_eq!(pin.revision, None);
}
