// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod shared_cache_isolation;
pub(super) mod tarball_builder;
pub(super) mod tarball_seed;
pub(in crate::bench_matrix) use shared_cache_isolation::with_isolated_shared_cache;
pub(in crate::bench_matrix) use tarball_builder::build_tarball_bytes;
pub(in crate::bench_matrix) use tarball_builder::sri_sha512;
pub(in crate::bench_matrix) use tarball_seed::seed_local_tarballs;
pub(in crate::bench_matrix) use tarball_seed::seed_shared_tarballs;
