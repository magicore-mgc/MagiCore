use crate::bench_matrix::types::ScenarioMeasurement;
// Benchmark baseline document type
// Kiểu dữ liệu chứa schema baseline đã lưu.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(in crate::bench_matrix) struct BenchmarkBaseline {
    pub(in crate::bench_matrix) generated_at_epoch_secs: u64,
    pub(in crate::bench_matrix) scenarios: Vec<ScenarioMeasurement>,
}
