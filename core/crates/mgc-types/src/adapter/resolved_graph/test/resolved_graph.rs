//! Resolved Graph nearest-owner contract tests.
//! Test contract gần owner resolved graph.

use crate::adapter::{ResolvedGraph, ResolvedPackage};
use crate::package::PackageId;

#[test]
fn resolved_graph_empty_creates_empty() {
    let g = ResolvedGraph::empty();
    assert_eq!(g.len(), 0);
    assert!(g.is_empty());
}

#[test]
fn resolved_graph_default_is_empty() {
    let g = ResolvedGraph::default();
    assert!(g.is_empty());
}

#[test]
fn resolved_graph_with_packages_not_empty() {
    let g = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::parse("foo@1.0.0").unwrap(),
            integrity: "sha1-xxx".to_string(),
            tarball_url: "https://example.com/pkg.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    assert_eq!(g.len(), 1);
    assert!(!g.is_empty());
}
