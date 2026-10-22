//! Cloud core project settings.
//! Cấu hình project dành riêng cho Cloud core.

use serde::{Deserialize, Serialize};

/// Cloud core config — `[cloud] type`.
/// Cấu hình loại công cụ Cloud đã chọn.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudConfig {
    /// Selected infrastructure type — loại công cụ hạ tầng đã chọn.
    #[serde(default)]
    pub r#type: String,
}

impl CloudConfig {
    /// Build Cloud settings from the selected infrastructure type.
    /// Tạo cấu hình Cloud theo loại hạ tầng đã chọn.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        Self {
            r#type: frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| "terraform".to_string()),
        }
    }
}
