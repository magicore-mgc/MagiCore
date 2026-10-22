//! Installed package identity and optional marker contracts.
//! Contract identity package cài và marker tùy chọn.

use mgc_types::{PackageId, PackageName, Version};

use super::super::{materialized_package_matches, write_materialized_package_marker};

fn package_id() -> PackageId {
    PackageId::new(
        PackageName::new("example").unwrap(),
        Version::parse("1.0.0").unwrap(),
    )
}

#[test]
fn absent_source_marker_does_not_create_a_target_marker() {
    let directory = tempfile::tempdir().unwrap();

    write_materialized_package_marker(directory.path(), None).unwrap();

    assert!(
        !directory
            .path()
            .join(".magicore-package-root.json")
            .exists()
    );
}

#[test]
fn installed_version_matches_when_no_source_marker_is_available() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("package.json"),
        br#"{"version":"1.0.0"}"#,
    )
    .unwrap();

    assert!(materialized_package_matches(directory.path(), &package_id(), None).unwrap());
}

#[test]
fn mismatched_installed_version_is_rejected_before_marker_checks() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("package.json"),
        br#"{"version":"2.0.0"}"#,
    )
    .unwrap();

    assert!(!materialized_package_matches(directory.path(), &package_id(), None).unwrap());
}
