// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod offline_cached_install;
pub(in crate::bench_matrix) use offline_cached_install::offline_cached_install;
