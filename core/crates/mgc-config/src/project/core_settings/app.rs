//! App core project settings.
//! Cấu hình project dành riêng cho App core.

use serde::{Deserialize, Serialize};

/// App core config — `[app]` language and optional platform set.
/// Cấu hình ngôn ngữ và danh sách nền tảng tùy chọn của App.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    /// Selected application language — ngôn ngữ ứng dụng đã chọn.
    #[serde(default)]
    pub language: String,
    /// Explicit platforms for multi-language apps — nền tảng chỉ định cho app đa ngôn ngữ.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<String>,
}

impl AppConfig {
    /// Build App settings, including the explicit `multi` platform default.
    /// Tạo cấu hình App, gồm default platform tường minh cho `multi`.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        let language = frameworks
            .first()
            .cloned()
            .unwrap_or_else(|| "flutter".to_string());
        let platforms = if language == "multi" {
            let rest: Vec<String> = frameworks
                .get(1..)
                .map(|items| items.to_vec())
                .unwrap_or_default();
            if rest.is_empty() {
                vec![
                    "android".to_string(),
                    "ios".to_string(),
                    "react-native".to_string(),
                    "flutter".to_string(),
                ]
            } else {
                rest
            }
        } else {
            Vec::new()
        };
        Self {
            language,
            platforms,
        }
    }
}
