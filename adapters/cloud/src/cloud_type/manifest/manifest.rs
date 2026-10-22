//! Classify a project for Cloud adapter selection — phân loại project khi chọn Cloud adapter.

use std::path::Path;

use super::super::detect_type;

pub(crate) fn manifest_is_cloud(root: &Path) -> bool {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(v) = toml::from_str::<toml::Value>(&content)
    {
        if v.get("ecosystem").and_then(|e| e.as_str()) == Some("cloud") {
            return true;
        }
        if v.get("cloud").is_some() {
            return true;
        }
    }
    detect_type(root).is_some()
}
