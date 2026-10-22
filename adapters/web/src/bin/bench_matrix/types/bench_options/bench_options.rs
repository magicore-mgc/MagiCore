use crate::bench_matrix::types::BenchmarkProfile;
// Benchmark command option type
// Kiểu cấu hình đã phân tích cho lượt chạy benchmark.

pub(in crate::bench_matrix) struct BenchOptions {
    pub(in crate::bench_matrix) runs: usize,
    pub(in crate::bench_matrix) save_baseline: Option<String>,
    pub(in crate::bench_matrix) compare_baseline: Option<String>,
    pub(in crate::bench_matrix) profile: BenchmarkProfile,
}
