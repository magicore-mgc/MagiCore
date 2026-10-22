//! Shared-cache root identity tests / test identity root của shared-cache.

use super::super::{has_exact_direct_root_selections, shared_cache_reuse_allowed};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, PackageId, PackageName, VersionRange,
    adapter::{ResolvedGraph, ResolvedPackage},
};

fn manifest_with_widget() -> Manifest {
    let mut manifest = Manifest::new("shared-cache-root-test", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("widget").expect("parse package name"),
            VersionRange::parse("^1.0.0").expect("parse version range"),
        ),
        false,
        false,
        false,
    );
    manifest
}

fn package(id: &str, direct: bool) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse(id).expect("parse package id"),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct,
        dev: false,
    }
}

#[test]
fn one_explicit_root_remains_distinct_from_a_matching_transitive_version() {
    let graph = ResolvedGraph {
        packages: vec![
            package("widget@1.2.0", true),
            package("widget@1.9.0", false),
        ],
    };

    assert!(has_exact_direct_root_selections(
        &manifest_with_widget(),
        &graph
    ));
}

#[test]
fn multiple_same_name_direct_candidates_force_a_cache_miss() {
    let graph = ResolvedGraph {
        packages: vec![package("widget@1.2.0", true), package("widget@1.9.0", true)],
    };

    assert!(!has_exact_direct_root_selections(
        &manifest_with_widget(),
        &graph
    ));
}

#[test]
fn missing_explicit_root_forces_a_cache_miss() {
    let graph = ResolvedGraph {
        packages: vec![package("widget@1.2.0", false)],
    };

    assert!(!has_exact_direct_root_selections(
        &manifest_with_widget(),
        &graph
    ));
}

#[test]
fn fresh_resolve_and_active_age_guard_both_disable_shared_cache_reuse() {
    assert!(!shared_cache_reuse_allowed(false, false));
    assert!(!shared_cache_reuse_allowed(true, true));
    assert!(shared_cache_reuse_allowed(true, false));
}
