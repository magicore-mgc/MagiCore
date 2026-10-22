// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod package_name;
pub(in crate::bench_matrix) use package_name::sanitize_pkg_name;
