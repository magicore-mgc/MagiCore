// Benchmark package graph fixture
// Tạo package ID, graph và resolved package cho fixture.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn package_id(name: &str, version: &str) -> PackageId {
    PackageId::new(
        PackageName::new(name).expect("valid package name"),
        Version::parse(version).expect("valid version"),
    )
}

pub(in crate::bench_matrix) fn make_graph(
    packages: &[PackageId],
    files_per_package: usize,
) -> ResolvedGraph {
    ResolvedGraph {
        packages: packages
            .iter()
            .cloned()
            .map(|id| resolved_pkg(id, vec![], true, files_per_package))
            .collect(),
    }
}

pub(in crate::bench_matrix) fn resolved_pkg(
    id: PackageId,
    deps: Vec<PackageId>,
    direct: bool,
    files_per_package: usize,
) -> ResolvedPackage {
    let bytes = build_tarball_bytes(&id, files_per_package)
        .expect("benchmark fixture tarball should build");
    ResolvedPackage {
        id,
        integrity: sri_sha512(&bytes),
        tarball_url: String::new(),
        deps,
        peer_deps: vec![],
        direct,
        dev: false,
    }
}
