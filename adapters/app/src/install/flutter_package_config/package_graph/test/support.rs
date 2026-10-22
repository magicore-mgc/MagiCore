//! Build the small resolved graph fixtures used by coordinator tests.
//! Dựng fixture resolved graph nhỏ cho test coordinator.

use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{PackageId, PackageName, Version};

pub(super) fn graph(names: &[&str]) -> ResolvedGraph {
    ResolvedGraph {
        packages: names
            .iter()
            .map(|name| ResolvedPackage {
                id: PackageId::new(
                    PackageName::new(*name).unwrap(),
                    Version::parse("1.0.0").unwrap(),
                ),
                integrity: String::new(),
                tarball_url: String::new(),
                deps: Vec::new(),
                peer_deps: Vec::new(),
                direct: false,
                dev: false,
            })
            .collect(),
    }
}
