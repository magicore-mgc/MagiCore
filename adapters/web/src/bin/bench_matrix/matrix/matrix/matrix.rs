// Benchmark matrix orchestration
// Khởi tạo runtime và giữ thứ tự fixture cùng sáu scenario.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn scenario_name(fixture: &FixtureSpec, suffix: &str) -> String {
    if fixture.scenario_prefix.is_empty() {
        suffix.to_string()
    } else {
        format!("{}-{suffix}", fixture.scenario_prefix)
    }
}

pub(in crate::bench_matrix) fn run_fixture_matrix(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<Vec<ScenarioMeasurement>> {
    let cold_local_cache = cold_local_cache(rt, runs, fixture)?;
    let warm_reinstall = warm_reinstall(rt, runs, fixture)?;
    let cold_online = cold_online(rt, runs, fixture)?;
    let shared_cache_bootstrap = shared_cache_bootstrap(rt, runs, fixture)?;
    let offline_cached_install = offline_cached_install(rt, runs, fixture)?;
    let repeated_cache_growth = repeated_cache_growth(rt, runs, fixture)?;
    Ok(vec![
        cold_local_cache.with_packages(fixture.graph.packages.len()),
        warm_reinstall.with_packages(fixture.graph.packages.len()),
        cold_online.with_packages(fixture.graph.packages.len()),
        shared_cache_bootstrap.with_packages(fixture.graph.packages.len()),
        offline_cached_install.with_packages(fixture.graph.packages.len()),
        repeated_cache_growth.with_packages(fixture.graph.packages.len()),
    ])
}

pub(in crate::bench_matrix) fn run_matrix(
    runs: usize,
    profile: BenchmarkProfile,
) -> anyhow::Result<Vec<ScenarioMeasurement>> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut scenarios = run_fixture_matrix(&rt, runs, &standard_fixture())?;
    if profile == BenchmarkProfile::Heavy {
        scenarios.extend(run_fixture_matrix(&rt, runs, &heavy_fixture())?);
    }
    Ok(scenarios)
}
