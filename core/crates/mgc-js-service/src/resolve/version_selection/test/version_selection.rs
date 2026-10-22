//! Exact constraints and project-root candidate preference.
//! Exact constraint và ưu tiên candidate project root.

use super::super::{exact_version_from_range, select_resolved_candidate};
use mgc_resolver::solver::Resolution;
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageId, PackageName, VersionRange};

fn resolution(package: &str) -> Resolution {
    let package_id = PackageId::parse(package).unwrap();
    Resolution {
        version: package_id.version().clone(),
        package_id,
        integrity: String::new(),
        deps: Vec::new(),
        dep_specs: Vec::new(),
        peer_deps: Vec::new(),
    }
}

#[test]
fn exact_version_requires_a_single_satisfying_version() {
    assert_eq!(
        exact_version_from_range(&VersionRange::parse("=1.2.3").unwrap())
            .unwrap()
            .to_string(),
        "1.2.3"
    );
    assert!(exact_version_from_range(&VersionRange::parse("^1.2.3").unwrap()).is_none());
    assert!(exact_version_from_range(&VersionRange::parse(">=1.2.3").unwrap()).is_none());
}

#[test]
fn compatible_manifest_root_precedes_a_newer_non_root_candidate() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let non_root = resolution("other@1.9.0");
    let root = resolution("react@1.2.0");
    let candidates = [&non_root, &root];

    let selected = select_resolved_candidate(
        &candidates,
        &VersionRange::parse("^1.0.0").unwrap(),
        &manifest,
    )
    .unwrap();

    assert_eq!(selected.package_id, root.package_id);
}
