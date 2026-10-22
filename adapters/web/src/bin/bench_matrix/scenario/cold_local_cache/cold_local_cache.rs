use crate::bench_matrix::scenario::measure_scenario;
// Cold local cache scenario
// Đo cài đặt với tarball cache local lạnh.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn cold_local_cache(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<ScenarioAccumulator> {
    measure_scenario(&scenario_name(fixture, "cold-local-cache"), runs, || {
        let dir = tempfile::tempdir()?;
        write_package_json(dir.path(), &fixture.direct_dependencies)?;
        seed_local_tarballs(
            dir.path(),
            &fixture.graph.packages,
            fixture.files_per_package,
        )?;
        with_isolated_shared_cache(None, |shared_root| {
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
    })
}
