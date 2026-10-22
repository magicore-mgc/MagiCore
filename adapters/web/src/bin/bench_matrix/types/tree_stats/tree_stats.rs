// Filesystem tree metrics type
// Kiểu dữ liệu metric tổng quát cho cây file.

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::bench_matrix) struct TreeStats {
    pub(in crate::bench_matrix) files: usize,
    pub(in crate::bench_matrix) bytes: u64,
}
