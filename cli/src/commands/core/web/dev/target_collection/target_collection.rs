//! Resolves ordered frontend/backend targets from shared Web workspace layout.
//! Resolve danh sách frontend/backend có thứ tự từ Web workspace layout dùng chung.

use anyhow::Result;
use std::path::Path;

use super::super::super::{
    WebProjectMode, detect_dev_target, detect_project_mode, workspace_backend_dir,
    workspace_frontend_dir,
};
use super::super::types::DevTarget;

pub(in crate::commands::core::web::implementation) fn dev_targets(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
) -> Result<Vec<DevTarget>> {
    // Backend port từ bảng trung tâm dev_port (RULE §13/§12 — không hardcode).
    // Backend port comes from the centralized dev_port table.
    let fullstack_backend_port = Some(crate::commands::core::dev_port::PORT_WEB_BE);
    let monorepo_backend_port = Some(crate::commands::core::dev_port::PORT_WEB_BE);

    match detect_project_mode(project_root)? {
        WebProjectMode::Standalone => Ok(vec![DevTarget {
            dir: detect_dev_target(project_root)?,
            role: "frontend",
            script_name: "dev",
            host,
            port,
        }]),
        WebProjectMode::FullstackSplit => Ok(vec![
            DevTarget {
                dir: project_root.to_path_buf(),
                role: "frontend",
                script_name: "dev:client",
                host,
                port,
            },
            DevTarget {
                dir: project_root.join("server"),
                role: "backend",
                script_name: "dev",
                host: None,
                port: fullstack_backend_port,
            },
        ]),
        WebProjectMode::Monorepo => Ok(vec![
            DevTarget {
                dir: workspace_frontend_dir(project_root)?,
                role: "frontend",
                script_name: "dev",
                host,
                port,
            },
            DevTarget {
                dir: workspace_backend_dir(project_root)?,
                role: "backend",
                script_name: "dev",
                host: None,
                port: monorepo_backend_port,
            },
        ]),
    }
}
