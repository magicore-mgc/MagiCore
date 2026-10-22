// Benchmark profile type
// Kiểu lựa chọn profile standard hoặc heavy.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bench_matrix) enum BenchmarkProfile {
    Standard,
    Heavy,
}
