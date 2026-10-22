// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod matrix;
pub(in crate::bench_matrix) use matrix::run_matrix;
pub(in crate::bench_matrix) use matrix::scenario_name;
