//! `mgc lib` facade tests: list, switch specs, active reporting.
//! Test facade `mgc lib`: list, spec switch, báo active.

use super::super::active::{ActiveState, write_active};
use super::super::error::KegError;
use super::super::layout::validate_version;
use super::super::lib_keg::{lib_active, lib_switch, lib_switch_spec, list_kegs, split_spec};
use super::support::Fixture;

fn seeded() -> Fixture {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.10.0");
    fixture.install("libcxx", "1.9.0");
    write_active(&fixture.layout, "libcxx", &ActiveState::initial("1.9.0")).unwrap();
    lib_switch(&fixture.store, "libcxx", "1.9.0").unwrap();
    fixture
}

#[test]
fn list_reports_versions_and_active() {
    let fixture = seeded();
    let listing = list_kegs(&fixture.store, "libcxx").unwrap();

    assert_eq!(
        listing.versions,
        vec![
            validate_version("1.9.0").unwrap(),
            validate_version("1.10.0").unwrap()
        ],
        "versions must sort semantically, not lexicographically"
    );
    assert_eq!(listing.active, Some(validate_version("1.9.0").unwrap()));
    assert_eq!(listing.previous, None);
}

#[test]
fn list_reports_active_without_a_keg_as_an_error() {
    let fixture = Fixture::new();
    write_active(&fixture.layout, "libcxx", &ActiveState::initial("3.0.0")).unwrap();

    let error = list_kegs(&fixture.store, "libcxx").unwrap_err();
    assert!(matches!(error, KegError::InvalidActive { .. }), "{error:?}");
}

#[test]
fn switch_spec_splits_on_the_last_at() {
    assert_eq!(split_spec("pkg@1.0.0").unwrap(), ("pkg", "1.0.0"));
    assert_eq!(
        split_spec("@scope/pkg@1.0.0").unwrap(),
        ("@scope/pkg", "1.0.0")
    );
    assert!(split_spec("pkg").is_err());
    assert!(split_spec("@1.0.0").is_err());
}

#[test]
fn switch_spec_activates_the_requested_version() {
    let fixture = seeded();
    let outcome = lib_switch_spec(&fixture.store, "libcxx@1.10.0").unwrap();
    assert_eq!(outcome.active, "1.10.0");
    assert_eq!(outcome.previous.as_deref(), Some("1.9.0"));
    assert_eq!(
        lib_active(&fixture.store, "libcxx")
            .unwrap()
            .unwrap()
            .version,
        "1.10.0"
    );
}

#[test]
fn unknown_library_lists_empty_and_has_no_active_version() {
    let fixture = Fixture::new();
    let listing = list_kegs(&fixture.store, "ghost").unwrap();
    assert!(listing.versions.is_empty());
    assert_eq!(listing.active, None);
    assert_eq!(lib_active(&fixture.store, "ghost").unwrap(), None);
}
