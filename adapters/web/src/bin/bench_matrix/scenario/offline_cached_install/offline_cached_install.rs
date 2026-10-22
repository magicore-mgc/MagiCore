use crate::bench_matrix::scenario::measure_scenario;
// Offline cached install scenario
// Đo install lại từ tarball đã cache khi registry không khả dụng.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn offline_cached_install(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<ScenarioAccumulator> {
    measure_scenario(
        &scenario_name(fixture, "offline-cached-install"),
        runs,
        || {
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
                rt.block_on(adapter.install(
                    &fixture.graph,
                    dir.path(),
                    mgc_types::adapter::InstallOptions::default(),
                ))?;
                std::fs::remove_dir_all(dir.path().join("node_modules"))?;
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
