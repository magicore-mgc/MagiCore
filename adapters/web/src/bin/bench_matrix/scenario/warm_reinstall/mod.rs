// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod warm_reinstall;
pub(in crate::bench_matrix) use warm_reinstall::warm_reinstall;
