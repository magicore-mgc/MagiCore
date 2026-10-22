//! Reads workspace package manifests and creates local workspace links.
//! Đọc manifest workspace package và tạo liên kết workspace cục bộ.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct WorkspacePackageJson {
    name: String,
    #[serde(default)]
    dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    dev_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "peerDependencies")]
    peer_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "optionalDependencies")]
    optional_dependencies: std::collections::HashMap<String, String>,
}

pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) fn link_monorepo_workspace_packages(
    project_root: &Path,
    targets: &[PathBuf],
) -> Result<()> {
    let package_targets = targets
        .iter()
        .filter(|path| path.join("package.json").exists())
        .cloned()
        .collect::<Vec<_>>();
    if package_targets.is_empty() {
        return Ok(());
    }

    let mut workspace_index = std::collections::HashMap::<String, PathBuf>::new();
    let mut manifests = Vec::new();

    for target in &package_targets {
        let manifest = read_workspace_package_manifest(target)?;
        workspace_index.insert(manifest.name.clone(), target.clone());
        manifests.push((target.clone(), manifest));
    }

    for (target, manifest) in manifests {
        let node_modules = target.join("node_modules");
        std::fs::create_dir_all(&node_modules)?;

        for (dep_name, spec) in manifest
            .dependencies
            .into_iter()
            .chain(manifest.dev_dependencies)
            .chain(manifest.peer_dependencies)
            .chain(manifest.optional_dependencies)
        {
            if !spec.trim().starts_with("workspace:") {
                continue;
            }
            let Some(source_dir) = workspace_index.get(&dep_name) else {
                return Err(crate::error::web_workspace_dep_missing(
                    &dep_name,
                    &target,
                    project_root,
                ));
            };

            let link_path = node_modules.join(dep_name.replace('/', std::path::MAIN_SEPARATOR_STR));
            if let Some(parent) = link_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if link_path.exists() {
                let metadata = std::fs::symlink_metadata(&link_path)?;
                if metadata.file_type().is_symlink() || metadata.is_file() {
                    std::fs::remove_file(&link_path)?;
                } else if metadata.is_dir() {
                    std::fs::remove_dir_all(&link_path)?;
                }
            }
            create_workspace_dir_link(source_dir, &link_path)?;
        }
    }

    Ok(())
}

fn read_workspace_package_manifest(project_root: &Path) -> Result<WorkspacePackageJson> {
    let path = project_root.join("package.json");
    let contents = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read workspace package manifest '{}'",
            path.display()
        )
    })?;
    serde_json::from_str(&contents).with_context(|| {
        format!(
            "failed to parse workspace package manifest '{}'",
            path.display()
        )
    })
}

#[cfg(unix)]
fn create_workspace_dir_link(source: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, target).with_context(|| {
        format!(
            "failed to symlink workspace package '{}' -> '{}'",
            target.display(),
            source.display()
        )
    })?;
    Ok(())
}

#[cfg(windows)]
fn create_workspace_dir_link(source: &Path, target: &Path) -> Result<()> {
    std::os::windows::fs::symlink_dir(source, target).with_context(|| {
        format!(
            "failed to symlink workspace package '{}' -> '{}'",
            target.display(),
            source.display()
        )
    })?;
    Ok(())
}
