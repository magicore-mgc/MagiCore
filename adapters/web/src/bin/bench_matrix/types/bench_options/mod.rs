// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod bench_options;
pub(in crate::bench_matrix) use bench_options::BenchOptions;
