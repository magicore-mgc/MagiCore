//! Game core project settings.
//! Cấu hình project dành riêng cho Game core.

use serde::{Deserialize, Serialize};

/// Game core config — `[game] engine`.
/// Cấu hình engine game đã chọn.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameConfig {
    /// Selected game engine — engine game đã chọn.
    #[serde(default)]
    pub engine: String,
}

impl GameConfig {
    /// Build Game settings from the selected scaffold engine.
    /// Tạo cấu hình Game theo engine scaffold đã chọn.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        Self {
            engine: frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| "bevy".to_string()),
        }
    }
}
