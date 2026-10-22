use crate::bench_matrix::baseline::acquire_baseline_lock;
use crate::bench_matrix::baseline::baseline_path;
use crate::bench_matrix::baseline::current_unix_secs;
use crate::bench_matrix::baseline::release_baseline_lock;
// Baseline writer
// Ghi baseline theo schema hiện tại và giữ luồng lock cũ.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn save_baseline(
    name: &str,
    scenarios: &[ScenarioMeasurement],
) -> anyhow::Result<()> {
    let path = baseline_path(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    acquire_baseline_lock(name)?;
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&BenchmarkBaseline {
            generated_at_epoch_secs: current_unix_secs(),
            scenarios: scenarios.to_vec(),
        })?,
    )?;
    release_baseline_lock(name)?;
    println!();
    println!("saved baseline: {}", path.display());
    Ok(())
}
