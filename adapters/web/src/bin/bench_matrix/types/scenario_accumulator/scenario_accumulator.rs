use crate::bench_matrix::types::NodeModulesStats;
use crate::bench_matrix::types::ScenarioMeasurement;
use crate::bench_matrix::types::TreeStats;
// Scenario sample accumulator type
// Kiểu gom mẫu đo trước khi tạo kết quả.

pub(in crate::bench_matrix) struct ScenarioAccumulator {
    pub(in crate::bench_matrix) name: String,
    pub(in crate::bench_matrix) samples_ms: Vec<f64>,
    pub(in crate::bench_matrix) bytes_from_cache: u64,
    pub(in crate::bench_matrix) stats: NodeModulesStats,
    pub(in crate::bench_matrix) cache_stats: TreeStats,
}

impl ScenarioAccumulator {
    pub(in crate::bench_matrix) fn with_packages(self, packages: usize) -> ScenarioMeasurement {
        let mut samples = self.samples_ms;
        samples.sort_by(|a, b| a.partial_cmp(b).expect("non-NaN samples"));
        let median_ms = samples[samples.len() / 2];
        let n = samples.len() as f64;
        let avg_ms = samples.iter().sum::<f64>() / n;
        let variance = samples.iter().map(|s| (s - avg_ms).powi(2)).sum::<f64>() / n;
        let stddev_ms = variance.sqrt();
        let min_ms = *samples.first().expect("non-empty samples");
        let max_ms = *samples.last().expect("non-empty samples");
        ScenarioMeasurement {
            name: self.name,
            runs: samples.len(),
            median_ms,
            avg_ms,
            stddev_ms,
            min_ms,
            max_ms,
            packages,
            bytes_from_cache: self.bytes_from_cache,
            node_modules_files: self.stats.files,
            node_modules_bytes: self.stats.bytes,
            hardlinked_files: self.stats.hardlinked_files,
            cache_files: self.cache_stats.files,
            cache_bytes: self.cache_stats.bytes,
        }
    }
}
