//! Behavior tests for the registry_pin owner.
//! Test hành vi cho owner registry_pin.

use super::super::*;
use mgc_types::Version;

#[test]
fn registry_pin_normalizes_identity_and_sets_only_version_state() {
    let version = Version::parse("1.4.2").unwrap();

    let pin = build_registry_pin("Scope/Lib", &version);

    assert_eq!(pin.identity, "scope.lib");
    assert_eq!(pin.version.as_deref(), Some("1.4.2"));
    assert_eq!(pin.revision, None);
    assert_eq!(pin.branch, None);
    assert_eq!(pin.location, None);
}
