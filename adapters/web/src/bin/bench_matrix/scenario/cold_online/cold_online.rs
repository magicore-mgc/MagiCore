use crate::bench_matrix::scenario::measure_scenario;
// Cold online registry scenario
// Đo resolve và install qua registry loopback fixture.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn cold_online(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<ScenarioAccumulator> {
    measure_scenario(
        &scenario_name(fixture, "cold-online-registry"),
        runs,
        || {
            let registry_fixture = rt.block_on(spawn_registry_fixture(
                &fixture.graph.packages,
                fixture.files_per_package,
            ))?;
            let dir = tempfile::tempdir()?;
            write_package_json(dir.path(), &fixture.direct_dependencies)?;
            with_isolated_shared_cache(None, |shared_root| {
                let adapter = WebAdapter::with_registry_and_shared_cache_root(
                    registry_fixture.registry_url.clone(),
                    shared_root.to_path_buf(),
                )?;
                let started = Instant::now();
                let manifest = rt.block_on(adapter.parse_manifest(dir.path()))?;
                let graph = rt.block_on(adapter.resolve(&manifest))?;
                let summary = rt.block_on(adapter.install(
                    &graph,
                    dir.path(),
                    mgc_types::adapter::InstallOptions::default(),
                ))?;
                let elapsed = started.elapsed().as_secs_f64() * 1000.0;
                let stats = inspect_node_modules(&dir.path().join("node_modules"))?;
                Ok((
                    elapsed,
                    summary.bytes_from_cache,
                    stats,
                    TreeStats::default(),
                ))
            })
        },
    )
}
