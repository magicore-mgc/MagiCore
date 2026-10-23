/// Per-task outcome in task order — kết quả từng task theo thứ tự task.
#[derive(Debug, Clone)]
pub struct ParallelResult {
    /// Task index — chỉ số task.
    pub index: usize,
    /// Exit code — mã thoát.
    pub exit_code: i32,
    /// Captured stdout — stdout đã thu.
    pub stdout: String,
    /// Captured stderr — stderr đã thu.
    pub stderr: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
