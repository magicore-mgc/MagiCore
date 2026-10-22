use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct WorkspaceConfig {
    mode: Option<String>,
    layout: Option<WorkspaceLayout>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceLayout {
    apps_dir: Option<String>,
    packages_dir: Option<String>,
}

pub(crate) fn discover_workspace_projects(project_root: &Path) -> Result<Option<Vec<PathBuf>>> {
    let Some(contents) = mgc_workspace::read_workspace_config(project_root)? else {
        return Ok(None);
    };

    let config: WorkspaceConfig = toml::from_str(&contents)?;
    if config.mode.as_deref() != Some("monorepo") {
        return Ok(None);
    }

    let apps_dir = config
        .layout
        .as_ref()
        .and_then(|layout| layout.apps_dir.as_deref())
        .unwrap_or("apps");
    let packages_dir = config
        .layout
        .as_ref()
        .and_then(|layout| layout.packages_dir.as_deref())
        .unwrap_or("packages");

    let apps_dir = mgc_workspace::resolve_workspace_layout_dir(project_root, apps_dir, "apps_dir")?;
    let packages_dir =
        mgc_workspace::resolve_workspace_layout_dir(project_root, packages_dir, "packages_dir")?;
    let mut workspaces = vec![];
    collect_installable_projects(apps_dir, &mut workspaces)?;
    collect_installable_projects(packages_dir, &mut workspaces)?;

    workspaces.sort();
    workspaces.dedup();
    Ok(Some(workspaces))
}

// workspace_package_name moved to dispatch/engine.rs (its sole consumer,
// next to both call sites) — it was dead in the lib target and private
// to the recursive dispatch filter logic.
// Hàm đã chuyển sang dispatch/engine.rs (caller duy nhất).

pub(in crate::commands::install) fn collect_installable_projects(
    root: PathBuf,
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    if !mgc_workspace::workspace_entry_is_directory(&root)? {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !mgc_workspace::workspace_entry_is_directory(&path)? {
            continue;
        }

        // Propagate invalid identity instead of treating it as an empty directory.
        // (Không nuốt identity lỗi thành thư mục rỗng rồi đệ quy xuyên qua nó.)
        if mgc_config::project::ProjectConfig::detect_core(&path)?.is_some() {
            out.push(path);
            continue;
        }

        collect_installable_projects(path, out)?;
    }

    Ok(())
}
