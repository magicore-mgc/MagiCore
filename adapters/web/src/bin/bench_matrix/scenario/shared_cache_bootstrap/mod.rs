// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod shared_cache_bootstrap;
pub(in crate::bench_matrix) use shared_cache_bootstrap::shared_cache_bootstrap;
