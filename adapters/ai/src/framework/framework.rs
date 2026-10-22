//! Stable public facade for AI framework detection.
//! Facade công khai ổn định cho nhận diện framework AI.

use std::path::Path;

pub use super::types::AiFramework;

/// Detect a supported AI framework using the existing config-source order.
/// Nhận diện framework AI hỗ trợ theo thứ tự nguồn config hiện hành.
pub fn detect_framework(root: &Path) -> Option<AiFramework> {
    super::detection::detect_framework(root)
}
