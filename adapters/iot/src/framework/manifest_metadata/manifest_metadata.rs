//! Read IoT identity and target from the shared project manifest safely.
//! Đọc identity và target IoT từ project manifest qua loader dùng chung.

use std::path::Path;

/// Return whether project metadata or supported markers identify IoT.
/// Trả về metadata hoặc marker hỗ trợ có nhận diện IoT hay không.
pub(crate) fn manifest_is_iot(root: &Path) -> bool {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value.get("ecosystem").and_then(|entry| entry.as_str()) == Some("iot") {
            return true;
        }
        if value.get("iot").is_some() {
            return true;
        }
    }
    root.join("platformio.ini").exists()
        || root.join("west.yml").exists()
        || root.join("Cargo.toml").exists()
}

/// Return an explicitly configured IoT target, if present.
/// Trả target IoT cấu hình tường minh nếu có.
pub(crate) fn target_from_manifest(root: &Path) -> Option<String> {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
        && let Some(target) = value
            .get("iot")
            .and_then(|entry| entry.get("target"))
            .and_then(|entry| entry.as_str())
    {
        return Some(target.to_string());
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
