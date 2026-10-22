//! Detect IoT framework using project config first and marker files second.
//! Nhận diện framework IoT ưu tiên config rồi đến marker file.

use crate::framework::IotFramework;
use std::path::Path;

/// Detect the configured or marker-file IoT framework.
/// Nhận diện framework IoT từ config hoặc marker file.
pub fn detect_framework(root: &Path) -> Option<IotFramework> {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value
            .get("ecosystem")
            .and_then(|entry| entry.as_str())
            .is_some_and(|ecosystem| ecosystem != "iot")
        {
            return None;
        }
        if let Some(framework) = value
            .get("iot")
            .and_then(|entry| entry.get("framework"))
            .and_then(|entry| entry.as_str())
            && let Some(framework) = IotFramework::from_str(framework)
        {
            return Some(framework);
        }
    }
    if root.join("platformio.ini").exists() {
        return Some(IotFramework::Platformio);
    }
    if root.join("west.yml").exists() {
        return Some(IotFramework::Zephyr);
    }
    if root.join("Cargo.toml").exists() {
        return Some(IotFramework::Esp32Rust);
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
