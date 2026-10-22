/// Exact read-only exception — ngoại lệ chỉ đọc chính xác.
#[derive(Debug, Clone)]
pub struct ReadOnlyException {
    /// Program basename, lowercase — basename chương trình, lowercase.
    pub program: String,
    /// Exact argument list — danh sách tham số chính xác.
    pub args_exact: Vec<String>,
    /// Manifest verified and cache unexpired — manifest đã xác minh còn hạn.
    pub unexpired: bool,
    /// MGC-synthesized answer; `None` means spawn the real invocation.
    /// Câu trả lời do MGC tổng hợp; `None` nghĩa là chạy invocation thật.
    pub answer: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
