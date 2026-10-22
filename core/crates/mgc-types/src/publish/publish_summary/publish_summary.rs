//! Represent the serializable result of publishing a package.
//! Biểu diễn kết quả publish package có thể serialize.

use serde::Serialize;

/// Summarize publish output for reuse by the CLI and publish pipeline.
/// Tóm tắt kết quả publish để CLI và pipeline publish có thể dùng lại.
#[derive(Debug, Clone, Serialize)]
pub struct PublishSummary {
    pub name: String,
    pub version: String,
    pub tag: String,
    pub size: u64,
    pub unpacked_size: u64,
    pub shasum: String,
    pub integrity: String,
    pub files: usize,
    pub entry_count: usize,
}
