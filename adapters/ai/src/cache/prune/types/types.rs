//! Public prune data types — kiểu policy và kết quả public của AI cache prune.

/// AI cache eviction policy.
/// Chính sách loại bỏ entry khỏi AI cache.
#[derive(Debug, Clone, Copy)]
pub enum PruneStrategy {
    /// Remove entries cached longer than this many days.
    /// Xóa entry được cache lâu hơn số ngày này.
    OlderThan(u64),
    /// Remove entries not accessed for this many days.
    /// Xóa entry không được truy cập trong số ngày này.
    UnusedFor(u64),
    /// Keep the N entries with the newest recorded last-access time.
    /// Giữ N entry có thời điểm truy cập gần nhất theo metadata.
    KeepRecent(usize),
    /// Remove entries whose measured logical content size exceeds this byte count.
    /// Xóa entry có logical content size đo được vượt số byte này.
    LargerThan(u64),
}

/// Summary of entries removed by a prune operation.
/// Tổng kết entry được xóa bởi một lần prune.
#[derive(Debug, Clone)]
pub struct PruneResult {
    pub removed_count: usize,
    /// Logical bytes from regular files removed, including regular metadata sidecars.
    /// Tổng logical bytes của file thường đã xóa, gồm sidecar metadata thường.
    pub bytes_freed: u64,
    pub removed_models: Vec<String>,
}
