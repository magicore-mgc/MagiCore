//! Stable public finding data returned by the GitHub Actions scanner.
//! Dữ liệu finding public ổn định do scanner GitHub Actions trả về.

/// Public finding shape for workflow policy violations.
/// Hình finding public cho vi phạm policy workflow.
#[derive(Debug, Clone)]
pub struct WorkflowFinding {
    pub file: String,
    pub line: usize,
    pub rule: &'static str,
    pub detail: String,
}

#[cfg(test)]
#[path = "types/test/types.rs"]
mod tests;
