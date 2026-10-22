// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod tarball_builder;
pub(in crate::bench_matrix) use tarball_builder::build_tarball_bytes;
pub(in crate::bench_matrix) use tarball_builder::sri_sha512;
