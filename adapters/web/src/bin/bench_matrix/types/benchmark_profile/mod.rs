// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod benchmark_profile;
pub(in crate::bench_matrix) use benchmark_profile::BenchmarkProfile;
