// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod package_graph;
pub(in crate::bench_matrix) use package_graph::make_graph;
pub(in crate::bench_matrix) use package_graph::package_id;
pub(in crate::bench_matrix) use package_graph::resolved_pkg;
