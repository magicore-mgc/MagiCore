#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Packages outside a tier predicate are not copied into its graph.
//! Package ngoài predicate tier không được đưa vào graph con.

use crate::native::rn_layers::graph_subset::graph_subset::subset;
use mgc_types::{PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};

#[test]
fn subset_keeps_only_matching_package_names() {
    let package = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("com.example:library").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: false,
        dev: false,
    };
    let graph = ResolvedGraph {
        packages: vec![package],
    };
    let subset = subset(&graph, |name| name.starts_with("com.example:"));
    assert_eq!(subset.packages.len(), 1);
}
