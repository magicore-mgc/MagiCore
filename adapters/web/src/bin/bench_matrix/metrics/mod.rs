// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod node_modules;
pub(super) mod tree;
pub(in crate::bench_matrix) use node_modules::inspect_node_modules;
pub(in crate::bench_matrix) use tree::inspect_tree;
