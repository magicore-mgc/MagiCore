use crate::bench_matrix::scenario::measure_scenario;
// Shared cache bootstrap scenario
// Đo cài đặt bootstrap từ shared cache fixture.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn shared_cache_bootstrap(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<ScenarioAccumulator> {
    measure_scenario(
        &scenario_name(fixture, "shared-cache-bootstrap"),
        runs,
        || {
            let shared = tempfile::tempdir()?;
            seed_shared_tarballs(
                shared.path(),
                &fixture.graph.packages,
                fixture.files_per_package,
            )?;
            let dir = tempfile::tempdir()?;
            write_package_json(dir.path(), &fixture.direct_dependencies)?;
            with_isolated_shared_cache(Some(shared.path()), |shared_root| {
                let adapter = WebAdapter::with_registry_and_shared_cache_root(
                    "http://127.0.0.1:9".into(),
                    shared_root.to_path_buf(),
                )?;
                let started = Instant::now();
                let summary = rt.block_on(adapter.install(
                    &fixture.graph,
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
