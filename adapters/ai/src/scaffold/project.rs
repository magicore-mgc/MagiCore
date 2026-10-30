//! Orchestrate AI project scaffolding per framework.
//! Điều phối scaffold project AI theo framework.

use std::path::Path;

use mgc_types::MgResult;

use crate::framework::AiFramework;

use super::layout::{ProjectLayout, pyproject_toml};

/// Write a new AI project skeleton for `engine` into `target_dir`.
/// Ghi skeleton project AI mới cho `engine` vào `target_dir`.
///
/// The destination must not exist yet: a create never merges into an existing
/// tree, and the caller reports the same conflict on its other source paths.
/// Đích chưa được tồn tại: lệnh create không hòa vào cây đã có, caller báo cùng
/// lỗi xung đột ở các đường nguồn khác.
pub async fn scaffold_project(
    engine: AiFramework,
    project_name: &str,
    target_dir: &Path,
) -> MgResult<()> {
    if target_dir.try_exists()? {
        return Err(anyhow::anyhow!("Directory '{}' already exists", target_dir.display()).into());
    }
    std::fs::create_dir_all(target_dir)?;

    let layout = ProjectLayout::new(target_dir, project_name);
    match engine {
        AiFramework::PythonAgent => super::python_agent::write(&layout).await?,
        AiFramework::McpServer => super::mcp_server::write(&layout).await?,
    }
    layout.write("pyproject.toml", &pyproject_toml(&layout, engine))?;
    Ok(())
}
