// Scenario measurement type
// Kiểu kết quả đo cho từng scenario.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(in crate::bench_matrix) struct ScenarioMeasurement {
    pub(in crate::bench_matrix) name: String,
    pub(in crate::bench_matrix) runs: usize,
    pub(in crate::bench_matrix) median_ms: f64,
    pub(in crate::bench_matrix) avg_ms: f64,
    pub(in crate::bench_matrix) stddev_ms: f64,
    pub(in crate::bench_matrix) min_ms: f64,
    pub(in crate::bench_matrix) max_ms: f64,
    pub(in crate::bench_matrix) packages: usize,
    pub(in crate::bench_matrix) bytes_from_cache: u64,
    pub(in crate::bench_matrix) node_modules_files: usize,
    pub(in crate::bench_matrix) node_modules_bytes: u64,
    pub(in crate::bench_matrix) hardlinked_files: usize,
    #[serde(default)]
    pub(in crate::bench_matrix) cache_files: usize,
    #[serde(default)]
    pub(in crate::bench_matrix) cache_bytes: u64,
}
