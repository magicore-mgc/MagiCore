use crate::bench_matrix::fixture::make_graph;
use crate::bench_matrix::fixture::package_id;
// Standard benchmark fixture
// Định nghĩa workload standard và package version đã đóng băng.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn standard_fixture() -> FixtureSpec {
    let direct_dependencies = vec![
        package_id("react", "18.2.0"),
        package_id("react-dom", "18.2.0"),
        package_id("vite", "5.4.10"),
        package_id("typescript", "5.5.4"),
        package_id("zod", "3.23.8"),
        package_id("tailwindcss", "4.3.2"),
    ];
    FixtureSpec {
        graph: make_graph(&direct_dependencies, 16),
        direct_dependencies,
        files_per_package: 16,
        scenario_prefix: "",
    }
}
