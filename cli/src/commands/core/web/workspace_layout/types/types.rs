//! Serde models for MagiCore workspace layout configuration.
//! Kiểu Serde cho cấu hình workspace layout của MagiCore.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(in crate::commands::core::web::implementation::workspace_layout) struct WorkspaceConfig {
    pub(in crate::commands::core::web::implementation::workspace_layout) mode: Option<String>,
    pub(in crate::commands::core::web::implementation::workspace_layout) layout:
        Option<WorkspaceLayout>,
    pub(in crate::commands::core::web::implementation::workspace_layout) workspace:
        Option<WorkspaceManifest>,
}

#[derive(Debug, Deserialize)]
pub(in crate::commands::core::web::implementation::workspace_layout) struct WorkspaceLayout {
    pub(in crate::commands::core::web::implementation::workspace_layout) apps_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::commands::core::web::implementation::workspace_layout) struct WorkspaceManifest {
    pub(in crate::commands::core::web::implementation::workspace_layout) apps: Option<Vec<String>>,
    pub(in crate::commands::core::web::implementation::workspace_layout) packages:
        Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub(in crate::commands::core::web) enum WebProjectMode {
    Standalone,
    FullstackSplit,
    Monorepo,
}
