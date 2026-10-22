// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod package_manifest;
pub(in crate::bench_matrix) use package_manifest::write_package_json;
