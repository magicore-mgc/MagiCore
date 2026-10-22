//! Contract tests for pinned dependency manifests and graph merging.
//! Test contract cho manifest ghim dependency và gộp graph.

use super::super::implementation::{AddedPackage, build_delta_manifest, merge_graphs};
use mgc_types::{
    Ecosystem, Manifest, PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version,
};

fn package(name: &str, version: &str) -> PackageId {
    PackageId::new(
        PackageName::new(name).expect("valid package name"),
        Version::parse(version).expect("valid package version"),
    )
}

#[test]
fn delta_manifest_pins_versions_and_preserves_dependency_groups() {
    let base = Manifest::new("demo", Ecosystem::Web);
    let additions = [
        AddedPackage {
            id: package("regular", "1.2.3"),
            dev: false,
            optional: false,
            peer: false,
        },
        AddedPackage {
            id: package("development", "2.3.4"),
            dev: true,
            optional: false,
            peer: false,
        },
        AddedPackage {
            id: package("optional", "3.4.5"),
            dev: true,
            optional: true,
            peer: false,
        },
        AddedPackage {
            id: package("peer", "4.5.6"),
            dev: true,
            optional: true,
            peer: true,
        },
    ];

    let delta = build_delta_manifest(&base, &additions).expect("valid pinned versions");

    assert_eq!(delta.dependencies.len(), 1);
    assert_eq!(delta.dependencies[0].name.as_str(), "regular");
    assert_eq!(delta.dependencies[0].range.as_str(), "=1.2.3");
    assert_eq!(delta.dev_dependencies.len(), 1);
    assert_eq!(delta.dev_dependencies[0].name.as_str(), "development");
    assert_eq!(delta.dev_dependencies[0].range.as_str(), "=2.3.4");
    assert_eq!(delta.optional_dependencies.len(), 1);
    assert_eq!(delta.optional_dependencies[0].name.as_str(), "optional");
    assert_eq!(delta.optional_dependencies[0].range.as_str(), "=3.4.5");
    assert_eq!(delta.peer_dependencies.len(), 1);
    assert_eq!(delta.peer_dependencies[0].name.as_str(), "peer");
    assert_eq!(delta.peer_dependencies[0].range.as_str(), "=4.5.6");
}

#[test]
fn graph_merge_preserves_existing_metadata_and_appends_only_new_edges() {
    let existing_id = package("shared", "1.0.0");
    let edge = package("edge", "1.0.0");
    let new_edge = package("new-edge", "2.0.0");
    let missing_metadata_id = package("missing-metadata", "2.0.0");
    let added_id = package("added", "3.0.0");
    let base = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: existing_id.clone(),
                integrity: "sha512-existing".to_string(),
                tarball_url: "https://registry.example/shared.tgz".to_string(),
                deps: vec![edge.clone()],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
            ResolvedPackage {
                id: missing_metadata_id.clone(),
                integrity: String::new(),
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
        ],
    };
    let delta = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: existing_id,
                integrity: "sha512-replacement".to_string(),
                tarball_url: "https://registry.example/replacement.tgz".to_string(),
                deps: vec![edge.clone(), new_edge.clone()],
                peer_deps: vec![],
                direct: true,
                dev: true,
            },
            ResolvedPackage {
                id: missing_metadata_id,
                integrity: "sha512-filled".to_string(),
                tarball_url: "https://registry.example/filled.tgz".to_string(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
            ResolvedPackage {
                id: added_id.clone(),
                integrity: "sha512-added".to_string(),
                tarball_url: "https://registry.example/added.tgz".to_string(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
        ],
    };

    let merged = merge_graphs(base, delta);

    assert_eq!(merged.packages.len(), 3);
    assert_eq!(merged.packages[0].id, package("shared", "1.0.0"));
    assert_eq!(merged.packages[0].integrity, "sha512-existing");
    assert_eq!(
        merged.packages[0].tarball_url,
        "https://registry.example/shared.tgz"
    );
    assert!(merged.packages[0].direct);
    assert!(merged.packages[0].dev);
    assert_eq!(merged.packages[0].deps, vec![edge, new_edge]);
    assert_eq!(merged.packages[1].integrity, "sha512-filled");
    assert_eq!(
        merged.packages[1].tarball_url,
        "https://registry.example/filled.tgz"
    );
    assert_eq!(merged.packages[2].id, added_id);
}
