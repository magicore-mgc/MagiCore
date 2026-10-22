//! Build small resolved-graph fixtures for projection tests.
//! Dựng fixture resolved graph nhỏ cho test projection.

use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{PackageId, PackageName, Version};

pub(super) fn package(name: &str, version: &str, dependencies: &[(&str, &str)]) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: dependencies
            .iter()
            .map(|&(dependency, version)| {
                PackageId::new(
                    PackageName::new(dependency).unwrap(),
                    Version::parse(version).unwrap(),
                )
            })
            .collect(),
        peer_deps: Vec::new(),
        direct: false,
        dev: false,
    }
}

pub(super) fn graph(packages: Vec<ResolvedPackage>) -> ResolvedGraph {
    ResolvedGraph { packages }
}
