// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod cold_local_cache;
pub(super) mod cold_online;
pub(super) mod measurement;
pub(super) mod offline_cached_install;
pub(super) mod repeated_cache_growth;
pub(super) mod shared_cache_bootstrap;
pub(super) mod warm_reinstall;
pub(in crate::bench_matrix) use cold_local_cache::cold_local_cache;
pub(in crate::bench_matrix) use cold_online::cold_online;
pub(in crate::bench_matrix) use measurement::measure_scenario;
pub(in crate::bench_matrix) use offline_cached_install::offline_cached_install;
pub(in crate::bench_matrix) use repeated_cache_growth::repeated_cache_growth;
pub(in crate::bench_matrix) use shared_cache_bootstrap::shared_cache_bootstrap;
pub(in crate::bench_matrix) use warm_reinstall::warm_reinstall;
