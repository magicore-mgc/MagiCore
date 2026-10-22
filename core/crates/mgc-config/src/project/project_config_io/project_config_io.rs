//! Project configuration load and atomic save operations.
//! Owner đọc và lưu cấu hình project theo thứ tự hiện có.

use std::path::Path;

use super::super::{
    ProjectConfig, atomic_write_project_config, default_execution_for, read_regular_project_text,
};

impl ProjectConfig {
    /// Load from the project root (`mgc.toml`).
    /// Đọc cấu hình từ thư mục gốc project.
    pub fn load(project_root: &Path) -> Result<Option<Self>, anyhow::Error> {
        let path = project_root.join("mgc.toml");
        let Some(content) = read_regular_project_text(&path, "project config")? else {
            return Ok(None);
        };
        let document: toml::Value = toml::from_str(&content)?;
        let mut config: Self = toml::from_str(&content)?;
        // Omitted execution metadata follows the same core defaults as project creation.
        // Thiếu bảng execution dùng default theo core; pin tường minh giữ nguyên.
        if document.get("execution").is_none() {
            config.execution = default_execution_for(&config.ecosystem, &config.features);
        }
        Ok(Some(config))
    }

    /// Save the configuration to the project root (`mgc.toml`).
    /// Lưu cấu hình vào thư mục gốc project.
    pub fn save(&self, project_root: &Path) -> Result<(), anyhow::Error> {
        let desired_core = Self::canonical_core(&self.ecosystem);
        // Claim the project core before writing config so a save from another
        // core cannot replace either the marker or the existing mgc.toml.
        // (Ghi nhận core trước để save từ core khác không thể thay marker hay mgc.toml.)
        Self::ensure_core_marker_at(project_root, &desired_core)?;
        let path = project_root.join("mgc.toml");
        let content = toml::to_string_pretty(self)?;
        atomic_write_project_config(&path, content.as_bytes())?;
        Ok(())
    }
}
