//! Root selection and unreachable-package pruning contracts.
//! Hợp đồng chọn root và loại package không reachable.

use super::super::{normalize_resolved_graph, prune_unreachable_packages};
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageId, PackageName, VersionRange};

fn package(id: &str, direct: bool, deps: Vec<PackageId>) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse(id).unwrap(),
        integrity: String::new(),
        tarball_url: String::new(),
        deps,
        peer_deps: Vec::new(),
        direct,
        dev: false,
    }
}

#[test]
fn normalization_keeps_the_unique_explicit_root_and_reachable_transitive_version() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("postcss").unwrap(),
            VersionRange::parse("^8.5.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("plugin").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let older = PackageId::parse("postcss@8.5.23").unwrap();
    let newer = PackageId::parse("postcss@8.5.28").unwrap();
    let graph = normalize_resolved_graph(
        &manifest,
        ResolvedGraph {
            packages: vec![
                package("postcss@8.5.23", true, Vec::new()),
                package("postcss@8.5.28", false, Vec::new()),
                package("plugin@1.0.0", true, vec![newer.clone()]),
            ],
        },
    );
    let by_id: std::collections::HashMap<_, _> = graph
        .packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect();

    assert!(by_id[&older].direct);
    assert!(!by_id[&newer].direct);
    assert!(by_id.contains_key(&newer));
}

#[test]
fn normalization_preserves_an_explicit_override_root_outside_the_manifest_range() {
    let mut manifest = Manifest::new("override-root", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("widget").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let override_root = PackageId::parse("widget@2.0.0").unwrap();
    let graph = normalize_resolved_graph(
        &manifest,
        ResolvedGraph {
            packages: vec![
                package("widget@2.0.0", true, Vec::new()),
                package("widget@1.9.0", false, Vec::new()),
            ],
        },
    );

    let direct: Vec<_> = graph
        .packages
        .iter()
        .filter(|package| package.direct)
        .map(|package| package.id.clone())
        .collect();

    assert_eq!(direct, vec![override_root]);
}

#[test]
fn pruning_removes_packages_without_a_path_from_any_direct_root() {
    let dependency = PackageId::parse("dep@1.0.0").unwrap();
    let mut graph = ResolvedGraph {
        packages: vec![
            package("root@1.0.0", true, vec![dependency.clone()]),
            package("dep@1.0.0", false, Vec::new()),
            package("orphan@1.0.0", false, Vec::new()),
        ],
    };

    prune_unreachable_packages(&mut graph);

    let names: Vec<_> = graph
        .packages
        .iter()
        .map(|package| package.id.name_str())
        .collect();
    assert_eq!(names, vec!["root", "dep"]);
}
