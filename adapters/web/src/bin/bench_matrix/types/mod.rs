// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod baseline;
pub(super) mod bench_options;
pub(super) mod benchmark_profile;
pub(super) mod fixture_spec;
pub(super) mod node_modules_stats;
pub(super) mod registry_fixture;
pub(super) mod scenario_accumulator;
pub(super) mod scenario_measurement;
pub(super) mod tree_stats;
pub(in crate::bench_matrix) use baseline::BenchmarkBaseline;
pub(in crate::bench_matrix) use bench_options::BenchOptions;
pub(in crate::bench_matrix) use benchmark_profile::BenchmarkProfile;
pub(in crate::bench_matrix) use fixture_spec::FixtureSpec;
pub(in crate::bench_matrix) use node_modules_stats::NodeModulesStats;
pub(in crate::bench_matrix) use registry_fixture::RegistryFixture;
pub(in crate::bench_matrix) use scenario_accumulator::ScenarioAccumulator;
pub(in crate::bench_matrix) use scenario_measurement::ScenarioMeasurement;
pub(in crate::bench_matrix) use tree_stats::TreeStats;
