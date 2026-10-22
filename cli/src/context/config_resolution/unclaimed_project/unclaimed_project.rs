//! Build project config when no saved config or signature establishes identity.
//! Tạo cấu hình khi chưa có config hoặc chữ ký nào xác lập identity.

use std::path::Path;

use mgc_config::project::ProjectConfig;

use super::super::project_name;

/// Build config for an existing but otherwise unclaimed project root.
/// Tạo config cho thư mục project chưa có identity.
pub(in crate::context::config_resolution) fn for_project_root(
    root: &Path,
    core: &str,
) -> ProjectConfig {
    ProjectConfig::new(project_name(root), core)
}

/// Build the synthetic config used with `--core` outside a detected project.
/// Tạo config tạm khi dùng `--core` ngoài project đã nhận diện.
pub(in crate::context::config_resolution) fn without_project_root(core: &str) -> ProjectConfig {
    ProjectConfig::new("project", core)
}

#[cfg(test)]
#[path = "test/unclaimed_project.rs"]
mod tests;
