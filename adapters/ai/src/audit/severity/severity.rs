//! Audit severity type for AI model findings.
//! Kiểu mức độ nghiêm trọng cho finding model AI.

/// Ordered severity levels for audit findings.
/// Các mức độ finding được sắp theo thứ tự ưu tiên.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
