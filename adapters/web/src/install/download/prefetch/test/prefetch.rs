//! Tests for Web tarball prefetch selection.
//! Test chọn package cho prefetch tarball Web.

use std::collections::HashSet;

use mgc_types::PackageId;

use super::{initial_prefetch_window, should_prefetch_package};

#[test]
fn schedules_package_when_not_in_skip_set() {
    let package = PackageId::parse("react@19.0.0").expect("valid package id");
    assert!(should_prefetch_package(&package, &HashSet::new()));
}

#[test]
fn does_not_schedule_package_in_skip_set() {
    let package = PackageId::parse("react@19.0.0").expect("valid package id");
    let skip = HashSet::from([package.clone()]);
    assert!(!should_prefetch_package(&package, &skip));
}

#[test]
fn prefetch_window_never_exceeds_configured_concurrency() {
    assert_eq!(initial_prefetch_window(100, 24), 24);
}

#[test]
fn prefetch_window_uses_all_packages_when_graph_is_smaller() {
    assert_eq!(initial_prefetch_window(3, 24), 3);
}

#[test]
fn prefetch_window_is_empty_for_an_empty_graph() {
    assert_eq!(initial_prefetch_window(0, 24), 0);
}

#[test]
fn prefetch_window_never_uses_zero_concurrency() {
    assert_eq!(initial_prefetch_window(3, 0), 1);
}
