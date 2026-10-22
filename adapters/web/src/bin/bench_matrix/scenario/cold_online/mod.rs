// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod cold_online;
pub(in crate::bench_matrix) use cold_online::cold_online;
