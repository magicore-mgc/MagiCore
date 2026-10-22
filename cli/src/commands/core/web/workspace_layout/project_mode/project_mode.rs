//! Resolves standalone/full-stack/monorepo mode and standard app directories.
//! Resolve mode standalone/full-stack/monorepo và thư mục app chuẩn.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::manifest_presence::has_backend_manifest;
use super::super::types::{WebProjectMode, WorkspaceConfig};

fn is_workspace_monorepo(config: &WorkspaceConfig) -> bool {
    if config.mode.as_deref() == Some("monorepo") {
        return true;
    }

    config.workspace.as_ref().is_some_and(|workspace| {
        workspace.apps.as_ref().is_some_and(|apps| !apps.is_empty())
            || workspace
                .packages
                .as_ref()
                .is_some_and(|packages| !packages.is_empty())
    })
}

pub(in crate::commands::core::web) fn workspace_frontend_dir(
    project_root: &Path,
) -> Result<PathBuf> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            let apps_dir = config
                .layout
                .as_ref()
                .and_then(|layout| layout.apps_dir.as_deref())
                .unwrap_or("apps");
            let apps_dir =
                mgc_workspace::resolve_workspace_layout_dir(project_root, apps_dir, "apps_dir")?;
            let frontend = apps_dir.join("frontend");
            mgc_workspace::workspace_entry_is_directory(&frontend)?;
            return Ok(frontend);
        }
    }
    Ok(project_root.join("apps").join("frontend"))
}

pub(in crate::commands::core::web) fn workspace_backend_dir(
    project_root: &Path,
) -> Result<PathBuf> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            let apps_dir = config
                .layout
                .as_ref()
                .and_then(|layout| layout.apps_dir.as_deref())
                .unwrap_or("apps");
            let apps_dir =
                mgc_workspace::resolve_workspace_layout_dir(project_root, apps_dir, "apps_dir")?;
            let backend = apps_dir.join("backend");
            mgc_workspace::workspace_entry_is_directory(&backend)?;
            return Ok(backend);
        }
    }
    Ok(project_root.join("apps").join("backend"))
}

pub(in crate::commands::core::web) fn detect_project_mode(
    project_root: &Path,
) -> Result<WebProjectMode> {
    if let Some(contents) = mgc_workspace::read_workspace_config(project_root)? {
        let config: WorkspaceConfig = toml::from_str(&contents)?;
        if is_workspace_monorepo(&config) {
            return Ok(WebProjectMode::Monorepo);
        }
    }

    if has_backend_manifest(&project_root.join("server")) {
        return Ok(WebProjectMode::FullstackSplit);
    }

    Ok(WebProjectMode::Standalone)
}
