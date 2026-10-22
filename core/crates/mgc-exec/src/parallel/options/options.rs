/// Parallel execution options — tùy chọn chạy song song.
#[derive(Debug, Clone)]
pub struct ParallelOptions {
    /// Stop remaining tasks when one fails (concurrently semantics).
    /// Dừng task còn lại khi một task fail.
    pub kill_others: bool,
    /// Prefix captured lines with `[index]` — thêm prefix `[index]` cho output.
    pub prefix_output: bool,
}

impl Default for ParallelOptions {
    fn default() -> Self {
        Self {
            kill_others: true,
            prefix_output: true,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
