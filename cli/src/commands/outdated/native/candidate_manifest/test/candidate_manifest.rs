//! Tests unconstrained candidate-manifest construction.
//! Kiểm tra cách tạo manifest candidate không giới hạn.

#![allow(clippy::unwrap_used)]

use super::super::latest_candidate_manifest;
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn latest_candidate_manifest_removes_constraints_from_all_dependency_groups() {
    let mut manifest = Manifest::new("fixture", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("runtime").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("dev-only").unwrap(),
            VersionRange::parse("~2.0").unwrap(),
        ),
        true,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("peer-only").unwrap(),
            VersionRange::parse("3.0.0").unwrap(),
        ),
        false,
        false,
        true,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("optional-only").unwrap(),
            VersionRange::parse("4.0.0").unwrap(),
        ),
        false,
        true,
        false,
    );

    let latest = latest_candidate_manifest(&manifest);

    assert_eq!(latest.dependencies.len(), 4);
    assert!(
        latest
            .dependencies
            .iter()
            .all(|dependency| dependency.range.is_star()
                && !dependency.dev
                && !dependency.optional
                && !dependency.peer)
    );
    assert!(latest.dev_dependencies.is_empty());
    assert!(latest.peer_dependencies.is_empty());
    assert!(latest.optional_dependencies.is_empty());
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(manifest.dev_dependencies.len(), 1);
    assert_eq!(manifest.peer_dependencies.len(), 1);
    assert_eq!(manifest.optional_dependencies.len(), 1);
    assert_eq!(manifest.dependencies[0].range.as_str(), "^1.0");
}
