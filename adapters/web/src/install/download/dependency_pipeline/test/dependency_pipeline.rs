//! Tests for Web dependency pipeline package selection.
//! Test chọn package cho pipeline dependency Web.

use std::collections::HashSet;

use mgc_types::PackageId;

use super::should_schedule_pipeline_package;

#[test]
fn schedules_packages_outside_skip_set() {
    let package = PackageId::parse("react@19.0.0").expect("valid package id");
    assert!(should_schedule_pipeline_package(&package, &HashSet::new()));
}

#[test]
fn excludes_packages_in_skip_set() {
    let package = PackageId::parse("react@19.0.0").expect("valid package id");
    let skip = HashSet::from([package.clone()]);
    assert!(!should_schedule_pipeline_package(&package, &skip));
}
