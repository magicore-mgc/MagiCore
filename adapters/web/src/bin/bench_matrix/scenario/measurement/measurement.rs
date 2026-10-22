// Scenario measurement accumulator
// Thu thập các lần chạy và lưu mẫu đo gần nhất.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn measure_scenario<F>(
    name: &str,
    runs: usize,
    mut run_once: F,
) -> anyhow::Result<ScenarioAccumulator>
where
    F: FnMut() -> anyhow::Result<(f64, u64, NodeModulesStats, TreeStats)>,
{
    let mut samples_ms = Vec::with_capacity(runs);
    let mut last_bytes_from_cache = 0u64;
    let mut last_stats = NodeModulesStats {
        files: 0,
        bytes: 0,
        hardlinked_files: 0,
    };
    let mut last_cache_stats = TreeStats::default();

    for _ in 0..runs {
        let (sample_ms, bytes_from_cache, stats, cache_stats) = run_once()?;
        samples_ms.push(sample_ms);
        last_bytes_from_cache = bytes_from_cache;
        last_stats = stats;
        last_cache_stats = cache_stats;
    }

    Ok(ScenarioAccumulator {
        name: name.to_string(),
        samples_ms,
        bytes_from_cache: last_bytes_from_cache,
        stats: last_stats,
        cache_stats: last_cache_stats,
    })
}
