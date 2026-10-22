//! Existing framework-config source precedence.
//! Thứ tự ưu tiên nguồn config framework hiện hành.

use std::path::Path;

use super::AiFramework;

/// Prefer a supported `mgc.toml` value and fall back to `pyproject.toml`.
/// Ưu tiên giá trị `mgc.toml` hợp lệ rồi fallback sang `pyproject.toml`.
pub(in crate::framework) fn detect_framework(root: &Path) -> Option<AiFramework> {
    super::framework_from_project_config(root).or_else(|| super::framework_from_pyproject(root))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
