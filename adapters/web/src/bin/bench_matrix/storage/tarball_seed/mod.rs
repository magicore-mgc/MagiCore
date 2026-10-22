// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod tarball_seed;
pub(in crate::bench_matrix) use tarball_seed::seed_local_tarballs;
pub(in crate::bench_matrix) use tarball_seed::seed_shared_tarballs;
