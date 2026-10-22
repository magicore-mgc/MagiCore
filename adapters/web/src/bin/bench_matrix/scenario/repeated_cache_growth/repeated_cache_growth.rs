use crate::bench_matrix::scenario::measure_scenario;
// Repeated cache growth scenario
// Đo cache growth qua nhiều lượt cài lặp.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn repeated_cache_growth(
    rt: &tokio::runtime::Runtime,
    runs: usize,
    fixture: &FixtureSpec,
) -> anyhow::Result<ScenarioAccumulator> {
    measure_scenario(
        &scenario_name(fixture, "repeated-cache-growth"),
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
                for _ in 0..2 {
                    rt.block_on(adapter.install(
                        &fixture.graph,
                        dir.path(),
                        mgc_types::adapter::InstallOptions::default(),
                    ))?;
                    let node_modules = dir.path().join("node_modules");
                    if node_modules.exists() {
                        std::fs::remove_dir_all(&node_modules)?;
                    }
                }
                let before = inspect_tree(shared.path())?;
                let started = Instant::now();
                let mut last_summary = None;
                for idx in 0..12 {
                    let summary = rt.block_on(adapter.install(
                        &fixture.graph,
                        dir.path(),
                        mgc_types::adapter::InstallOptions::default(),
                    ))?;
                    last_summary = Some(summary);
                    let node_modules = dir.path().join("node_modules");
                    if idx < 11 && node_modules.exists() {
                        std::fs::remove_dir_all(node_modules)?;
                    }
                }
                let elapsed = started.elapsed().as_secs_f64() * 1000.0;
                let summary =
                    last_summary.ok_or_else(|| anyhow::anyhow!("repeated install did not run"))?;
                let after = inspect_tree(shared.path())?;
                let cache_growth = TreeStats {
                    files: after.files.saturating_sub(before.files),
                    bytes: after.bytes.saturating_sub(before.bytes),
                };
                let stats = inspect_node_modules(&dir.path().join("node_modules"))?;
                Ok((elapsed, summary.bytes_from_cache, stats, cache_growth))
            })
        },
    )
}
