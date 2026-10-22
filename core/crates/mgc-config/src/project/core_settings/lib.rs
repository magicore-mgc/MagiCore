//! Library core project settings.
//! Cấu hình project dành riêng cho Library core.

use serde::{Deserialize, Serialize};

/// Library core config — `[lib]` language and pip package allowlist.
/// Cấu hình ngôn ngữ và allowlist package pip của Library.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibConfig {
    /// Selected library language — ngôn ngữ thư viện đã chọn.
    #[serde(default)]
    pub language: String,
    /// Explicit Python dependency allowlist — allowlist dependency Python chỉ định.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pip_allowed_packages: Vec<String>,
}

impl LibConfig {
    /// Build the Lib settings selected by the project's scaffold language.
    /// Tạo cấu hình Lib theo ngôn ngữ scaffold đã chọn.
    pub(crate) fn for_scaffold(frameworks: &[String]) -> Self {
        let language = frameworks
            .first()
            .map(|framework| match framework.as_str() {
                "typescript" | "ts" => "ts",
                "python" | "py" => "python",
                "go" | "golang" => "go",
                "dotnet" | "csharp" | "c#" => "dotnet",
                "java" | "maven" => "java",
                _ => "rust",
            })
            .unwrap_or("rust")
            .to_string();
        Self {
            language,
            pip_allowed_packages: Vec::new(),
        }
    }
}
