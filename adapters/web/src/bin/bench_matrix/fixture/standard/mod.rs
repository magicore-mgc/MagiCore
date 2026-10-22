// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod standard;
pub(in crate::bench_matrix) use standard::standard_fixture;
