use crate::bench_matrix::baseline::acquire_baseline_lock;
use crate::bench_matrix::baseline::baseline_path;
use crate::bench_matrix::baseline::release_baseline_lock;
// Baseline reader
// Đọc baseline dưới lock và deserialize schema hiện có.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn load_baseline(name: &str) -> anyhow::Result<BenchmarkBaseline> {
    acquire_baseline_lock(name)?;
    let path = baseline_path(name);
    let baseline: BenchmarkBaseline = serde_json::from_slice(&std::fs::read(&path)?)?;
    release_baseline_lock(name)?;
    Ok(baseline)
}
