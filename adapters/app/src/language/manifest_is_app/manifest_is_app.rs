//! Classifies an App project from config or existing language detection.
//! Phân loại project App từ config hoặc bộ nhận diện language hiện tại.

use std::path::Path;

use super::super::detect_language;

/// Preserves config-first App classification and marker fallback behavior.
/// Giữ phân loại App ưu tiên config và fallback marker hiện tại.
pub(crate) fn manifest_is_app(root: &Path) -> bool {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value.get("ecosystem").and_then(toml::Value::as_str) == Some("app") {
            return true;
        }
        if value.get("app").is_some() {
            return true;
        }
    }
    detect_language(root).is_some()
}
