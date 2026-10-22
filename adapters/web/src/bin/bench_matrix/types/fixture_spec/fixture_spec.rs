// Benchmark fixture specification type
// Kiểu mô tả dependency graph và workload fixture.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) struct FixtureSpec {
    pub(in crate::bench_matrix) direct_dependencies: Vec<PackageId>,
    pub(in crate::bench_matrix) graph: ResolvedGraph,
    pub(in crate::bench_matrix) files_per_package: usize,
    pub(in crate::bench_matrix) scenario_prefix: &'static str,
}
