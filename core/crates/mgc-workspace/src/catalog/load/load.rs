//! Reads the current workspace catalog section from its TOML file.
//! Đọc phần catalog workspace hiện có từ file TOML.

use super::super::WorkspaceCatalogs;
use std::path::Path;

/// Load catalogs from magicore.workspace.toml when the file exists.
/// Nạp catalogs từ magicore.workspace.toml nếu file tồn tại.
pub fn load_workspace_catalogs(project_root: &Path) -> anyhow::Result<Option<WorkspaceCatalogs>> {
    let ws_path = project_root.join("magicore.workspace.toml");
    if !ws_path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&ws_path)?;
    #[derive(serde::Deserialize)]
    struct Config {
        #[serde(default)]
        catalogs: Option<WorkspaceCatalogs>,
    }
    let parsed: Config = toml::from_str(&contents)?;
    Ok(parsed.catalogs)
}
