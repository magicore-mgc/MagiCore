//! CICD core project settings.
//! Cấu hình project dành riêng cho CICD core.

use serde::{Deserialize, Serialize};

/// CICD core config — `[cicd] provider`.
/// Cấu hình nhà cung cấp CI/CD đã chọn.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CicdConfig {
    /// Selected CI/CD provider — nhà cung cấp CI/CD đã chọn.
    #[serde(default)]
    pub provider: String,
}

impl CicdConfig {
    /// Build CICD settings from the selected provider.
    /// Tạo cấu hình CICD theo provider đã chọn.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        Self {
            provider: frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| "github-actions".to_string()),
        }
    }
}
