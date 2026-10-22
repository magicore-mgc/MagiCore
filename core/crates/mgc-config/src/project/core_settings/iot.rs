//! IoT core project settings.
//! Cấu hình project dành riêng cho IoT core.

use serde::{Deserialize, Serialize};

/// IoT core config — `[iot]` framework and board.
/// Cấu hình framework và bo mạch IoT.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotConfig {
    /// Selected IoT framework — framework IoT đã chọn.
    #[serde(default)]
    pub framework: String,
    /// Selected target board — bo mạch mục tiêu đã chọn.
    #[serde(default)]
    pub board: String,
}

impl IotConfig {
    /// Build IoT settings while keeping the board fallback beside its owner.
    /// Tạo cấu hình IoT và giữ fallback board cạnh owner của nó.
    pub(crate) fn for_scaffold(frameworks: &[String], features: &[String]) -> Self {
        let framework = frameworks
            .first()
            .cloned()
            .unwrap_or_else(|| "esp32-rust".to_string());
        let board = features
            .first()
            .cloned()
            .unwrap_or_else(|| default_iot_board(Some(&framework)).to_string());
        Self { framework, board }
    }
}

fn default_iot_board(framework: Option<&str>) -> &'static str {
    match framework {
        Some("esp32-rust") => "esp32c3",
        Some("platformio") => "esp32dev",
        _ => "nrf52dk_nrf52832",
    }
}
