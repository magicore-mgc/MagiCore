//! Reads a parseable explicit App language from mgc.toml.
//! Đọc language App tường minh, parse được từ mgc.toml.

use std::path::Path;

/// Returns the configured string only when the config path and TOML value are readable.
/// Trả chuỗi cấu hình khi đường dẫn và giá trị TOML đọc/parse được.
pub(in crate::language) fn read(root: &Path) -> Option<String> {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
        && let Some(language) = value
            .get("app")
            .and_then(|app| app.get("language"))
            .and_then(toml::Value::as_str)
    {
        return Some(language.to_string());
    }
    None
}
