//! Direct root projection tests / Test chiếu package gốc trực tiếp.
#![allow(clippy::unwrap_used)]

use super::super::direct_package_ids;
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{PackageId, PackageName, Version};

fn resolved_package(name: &str, direct: bool) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse("1.0.0").unwrap(),
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
fn direct_root_projection_filters_transitives_and_preserves_graph_order() {
    let graph = ResolvedGraph {
        packages: vec![
            resolved_package("first-root", true),
            resolved_package("transitive", false),
            resolved_package("second-root", true),
        ],
    };

    assert_eq!(
        direct_package_ids(&graph),
        vec!["first-root@1.0.0", "second-root@1.0.0"]
    );
}
