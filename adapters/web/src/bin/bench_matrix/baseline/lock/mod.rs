// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod lock;
pub(in crate::bench_matrix) use lock::acquire_baseline_lock;
pub(in crate::bench_matrix) use lock::release_baseline_lock;
