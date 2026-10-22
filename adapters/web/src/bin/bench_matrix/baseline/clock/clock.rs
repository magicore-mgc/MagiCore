// Baseline timestamp source
// Tạo epoch timestamp cho schema baseline hiện có.
pub(in crate::bench_matrix) fn current_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
