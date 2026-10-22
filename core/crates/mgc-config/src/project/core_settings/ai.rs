//! AI core project settings.
//! Cấu hình project dành riêng cho AI core.

use serde::{Deserialize, Serialize};

/// AI core config — `[ai] framework` (python-agent/mcp-server).
/// Cấu hình framework AI đã chọn.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiConfig {
    /// Selected AI framework — framework AI đã chọn.
    #[serde(default)]
    pub framework: String,
}

impl AiConfig {
    /// Build AI settings from the selected agent framework.
    /// Tạo cấu hình AI theo agent framework đã chọn.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        Self {
            framework: frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| "python-agent".to_string()),
        }
    }
}
