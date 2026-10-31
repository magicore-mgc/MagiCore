//! Resolves ordered frontend/backend targets from shared Web workspace layout.
//! Resolve danh sách frontend/backend có thứ tự từ Web workspace layout dùng chung.

use anyhow::Result;
use std::path::Path;

use super::super::super::{
    WebProjectMode, detect_dev_target, detect_project_mode, workspace_backend_dir,
    workspace_frontend_dir,
};
use super::super::is_backend_half;
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
        // A server half without a start command is reported by the backend
        // resolver, not dropped here: silently running the frontend alone is
        // exactly the gap this target exists to close.
        // (Nửa server không có lệnh start do resolver backend báo, không bị bỏ ở
        // đây: chạy riêng frontend im lặng chính là lỗ hổng target này lấp.)
        WebProjectMode::FullstackSplit => {
            let server_dir = project_root.join("server");
            debug_assert!(
                is_backend_half(&server_dir),
                "FullstackSplit is only reported when server/ holds a backend manifest"
            );
            Ok(vec![
                DevTarget {
                    dir: project_root.to_path_buf(),
                    role: "frontend",
                    script_name: frontend_dev_script(project_root)?,
                    host,
                    port,
                },
                DevTarget {
                    dir: server_dir,
                    role: "backend",
                    script_name: "dev",
                    host: None,
                    port: fullstack_backend_port,
                },
            ])
        }
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

/// Script name that starts the frontend half of a split project.
/// Tên script khởi động nửa frontend của project tách rời.
///
/// Fullstack layouts name it `dev:client`, while a composed combo keeps the
/// framework's own `dev`. Asking the manifest which one exists keeps every
/// project that already declared `dev:client` working and stops a composed combo
/// from dead-ending on a script nobody ever seeds.
/// Bố cục fullstack gọi nó là `dev:client`, còn combo ghép giữ `dev` của chính
/// framework. Hỏi manifest xem tên nào có để project đã khai `dev:client` vẫn
/// chạy và combo ghép không chết ở script mà không ai seed.
fn frontend_dev_script(project_root: &Path) -> Result<&'static str> {
    if super::super::read_script(project_root, FRONTEND_DEV_CLIENT)?.is_some() {
        return Ok(FRONTEND_DEV_CLIENT);
    }
    if super::super::read_script(project_root, FRONTEND_DEV)?.is_some() {
        return Ok(FRONTEND_DEV);
    }
    Err(crate::error::web_no_dev_entrypoint(project_root))
}

/// Fullstack layouts' dedicated frontend script.
/// Script frontend riêng của bố cục fullstack.
const FRONTEND_DEV_CLIENT: &str = "dev:client";

/// The framework's own dev script.
/// Script dev của chính framework.
const FRONTEND_DEV: &str = "dev";
