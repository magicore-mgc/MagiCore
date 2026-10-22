// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod repeated_cache_growth;
pub(in crate::bench_matrix) use repeated_cache_growth::repeated_cache_growth;
