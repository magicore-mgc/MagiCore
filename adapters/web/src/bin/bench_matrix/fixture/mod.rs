// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod heavy;
pub(super) mod package_graph;
pub(super) mod package_manifest;
pub(super) mod package_name;
pub(super) mod registry;
pub(super) mod standard;
pub(in crate::bench_matrix) use heavy::heavy_fixture;
pub(in crate::bench_matrix) use package_graph::make_graph;
pub(in crate::bench_matrix) use package_graph::package_id;
pub(in crate::bench_matrix) use package_graph::resolved_pkg;
pub(in crate::bench_matrix) use package_manifest::write_package_json;
pub(in crate::bench_matrix) use package_name::sanitize_pkg_name;
pub(in crate::bench_matrix) use registry::spawn_registry_fixture;
pub(in crate::bench_matrix) use standard::standard_fixture;
