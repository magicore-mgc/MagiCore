//! Root membership and resolved-manifest projection contracts.
//! Hợp đồng nhận diện root và projection manifest đã resolve.

use super::super::{is_manifest_root_package, manifest_with_resolved_specs};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageId, PackageName, VersionRange};

fn manifest_with_dependencies() -> Manifest {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("zod").unwrap(),
            VersionRange::parse("^3.0.0").unwrap(),
        ),
        true,
        false,
        false,
    );
    manifest
}

#[test]
fn root_membership_requires_matching_name_and_range() {
    let manifest = manifest_with_dependencies();

    assert!(is_manifest_root_package(
        &manifest,
        &PackageId::parse("react@18.2.0").unwrap()
    ));
    assert!(!is_manifest_root_package(
        &manifest,
        &PackageId::parse("react@19.0.0").unwrap()
    ));
    assert!(!is_manifest_root_package(
        &manifest,
        &PackageId::parse("other@18.2.0").unwrap()
    ));
}

#[test]
fn resolved_specs_follow_manifest_dependency_group_order() {
    let manifest = manifest_with_dependencies();
    let resolved = manifest_with_resolved_specs(
        &manifest,
        &[
            (PackageName::new("react").unwrap(), "18.2.0".to_string()),
            (PackageName::new("zod").unwrap(), "3.22.4".to_string()),
        ],
    )
    .unwrap();

    assert_eq!(resolved.find_dep("react").unwrap().range.as_str(), "18.2.0");
    assert_eq!(resolved.find_dep("zod").unwrap().range.as_str(), "3.22.4");
    assert_eq!(resolved.dependencies.len(), 1);
    assert_eq!(resolved.dev_dependencies.len(), 1);
}

#[test]
fn resolved_spec_name_mismatch_is_rejected() {
    let error = manifest_with_resolved_specs(
        &manifest_with_dependencies(),
        &[
            (PackageName::new("zod").unwrap(), "3.22.4".to_string()),
            (PackageName::new("react").unwrap(), "18.2.0".to_string()),
        ],
    )
    .expect_err("resolved package order must match the manifest");

    assert!(
        error
            .to_string()
            .contains("does not match manifest dependency")
    );
}
