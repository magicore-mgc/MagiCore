// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod clock;
pub(super) mod load;
pub(super) mod lock;
pub(super) mod path;
pub(super) mod save;
pub(in crate::bench_matrix) use clock::current_unix_secs;
pub(in crate::bench_matrix) use load::load_baseline;
pub(in crate::bench_matrix) use lock::acquire_baseline_lock;
pub(in crate::bench_matrix) use lock::release_baseline_lock;
pub(in crate::bench_matrix) use path::baseline_path;
pub(in crate::bench_matrix) use save::save_baseline;
