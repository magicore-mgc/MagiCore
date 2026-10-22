//! Tests native direct-package graph comparison.
//! Kiểm tra so sánh graph package trực tiếp native.

#![allow(clippy::unwrap_used)]

use crate::commands::outdated::native::graph_compare::native_outdated_packages;
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, PackageId, PackageName, Version, VersionRange,
};

fn package(name: &str, version: &str, direct: bool) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct,
        dev: false,
    }
}
#[test]
fn native_outdated_compares_locked_direct_packages_and_ignores_transitives() {
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
    let current = ResolvedGraph {
        packages: vec![
            package("runtime", "1.4.0", true),
            package("transitive", "4.0.0", false),
        ],
    };
    let latest = ResolvedGraph {
        packages: vec![
            package("runtime", "2.0.0", true),
            package("transitive", "8.0.0", false),
        ],
    };

    let outdated = native_outdated_packages(&manifest, &current, &latest).unwrap();

    assert_eq!(outdated.len(), 1);
    assert_eq!(outdated[0].name, "runtime");
    assert_eq!(outdated[0].current, "1.4.0");
    assert_eq!(outdated[0].latest, "2.0.0");
}

#[test]
fn native_outdated_reports_a_package_once_when_declared_in_multiple_groups() {
    let mut manifest = Manifest::new("fixture", Ecosystem::Lib);
    manifest.dependencies.push(DependencySpec::new(
        PackageName::new("shared").unwrap(),
        VersionRange::parse("^1.0").unwrap(),
    ));
    manifest.dev_dependencies.push(DependencySpec::new(
        PackageName::new("shared").unwrap(),
        VersionRange::parse("~1.2").unwrap(),
    ));
    let current = ResolvedGraph {
        packages: vec![package("shared", "1.4.0", true)],
    };
    let latest = ResolvedGraph {
        packages: vec![package("shared", "2.0.0", true)],
    };

    let outdated = native_outdated_packages(&manifest, &current, &latest).unwrap();

    assert_eq!(outdated.len(), 1);
    assert_eq!(outdated[0].name, "shared");
}

#[test]
fn native_outdated_fails_when_a_direct_lock_pin_is_missing() {
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
    let latest = ResolvedGraph {
        packages: vec![package("runtime", "2.0.0", true)],
    };

    let error = native_outdated_packages(&manifest, &ResolvedGraph::empty(), &latest)
        .expect_err("missing locked current version must not look up to date");

    assert!(error.to_string().contains("runtime"));
}

#[test]
fn native_outdated_fails_when_a_fresh_direct_pin_is_missing() {
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
    let current = ResolvedGraph {
        packages: vec![package("runtime", "1.4.0", true)],
    };
    let error = native_outdated_packages(&manifest, &current, &ResolvedGraph::empty())
        .expect_err("missing latest direct pin must not appear current");
    assert!(
        error
            .to_string()
            .contains("did not return exactly one latest version")
    );
}

#[test]
fn native_outdated_refuses_downgrade_candidates() {
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
    let current = ResolvedGraph {
        packages: vec![package("runtime", "2.0.0", true)],
    };
    let latest = ResolvedGraph {
        packages: vec![package("runtime", "1.9.0", true)],
    };
    let error = native_outdated_packages(&manifest, &current, &latest)
        .expect_err("resolver downgrade must fail closed");
    assert_eq!(
        error.to_string(),
        "native outdated resolver returned 1.9.0 below locked version 2.0.0 for 'runtime'; refusing to report a downgrade"
    );
}

#[test]
fn native_outdated_rejects_ambiguous_current_direct_pins() {
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
    let current = ResolvedGraph {
        packages: vec![
            package("runtime", "1.4.0", true),
            package("runtime", "1.5.0", true),
        ],
    };
    let latest = ResolvedGraph {
        packages: vec![package("runtime", "2.0.0", true)],
    };
    let error = native_outdated_packages(&manifest, &current, &latest)
        .expect_err("multiple current direct pins must fail closed");
    assert!(error.to_string().contains("(found 2)"));
}

#[test]
fn native_outdated_results_are_sorted_by_package_name() {
    let mut manifest = Manifest::new("fixture", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("zeta").unwrap(),
            VersionRange::parse("^1").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("alpha").unwrap(),
            VersionRange::parse("^1").unwrap(),
        ),
        false,
        false,
        false,
    );
    let current = ResolvedGraph {
        packages: vec![
            package("zeta", "1.0.0", true),
            package("alpha", "1.0.0", true),
        ],
    };
    let latest = ResolvedGraph {
        packages: vec![
            package("zeta", "2.0.0", true),
            package("alpha", "2.0.0", true),
        ],
    };
    let outdated = native_outdated_packages(&manifest, &current, &latest).unwrap();
    assert_eq!(
        outdated
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        ["alpha", "zeta"]
    );
}
