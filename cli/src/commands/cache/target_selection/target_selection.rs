//! Select cache targets and their paths.
//! Chọn target và xác định đường dẫn cache tương ứng.

use crate::commands::cache::{CacheEntry, CacheTarget};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(in crate::commands::cache) fn cache_entries(
    target: CacheTarget,
    core: Option<&str>,
    include_build: bool,
) -> Result<Vec<CacheEntry>> {
    let mut entries = Vec::new();

    // Combine predicates with Rust 2024 let-chain syntax for clippy 1.98.
    // Gộp điều kiện theo cú pháp let-chain của Rust 2024 để đạt clippy 1.98.
    if target.includes(CacheTarget::Shared) {
        let data_root = mgc_platform::paths::mgc_data_dir();
        let shared = shared_cache_path(&data_root, core);
        entries.push(CacheEntry {
            label: "shared",
            path: shared,
            removable: true,
        });
        if core.is_none() || core == Some("app") {
            entries.push(CacheEntry {
                label: "app-pub-store",
                path: data_root.join("store").join("pub"),
                removable: true,
            });
            entries.push(CacheEntry {
                label: "app-swift-store",
                path: data_root.join("store").join("swift"),
                removable: true,
            });
        }
    }

    // Combine predicates with Rust 2024 let-chain syntax for clippy 1.98.
    // Gộp điều kiện theo cú pháp let-chain của Rust 2024 để đạt clippy 1.98.
    if target.includes(CacheTarget::Project)
        && let Ok(cwd) = std::env::current_dir()
        && let Some(root) = crate::commands::core::shared::find_project_root(&cwd)?
    {
        let project = match core {
            Some("web") => root.join(".magicore").join("cache").join("web"),
            Some(core) => root.join(".magicore").join("cache").join(core),
            None => root.join(".magicore").join("cache"),
        };
        entries.push(CacheEntry {
            label: "project",
            path: project,
            removable: true,
        });
    }

    if include_build {
        let build = workspace_build_cache_path()?;
        entries.push(CacheEntry {
            label: "build",
            path: build,
            removable: true,
        });
    }

    Ok(entries)
}

pub(in crate::commands::cache) fn shared_cache_path(
    data_root: &Path,
    core: Option<&str>,
) -> PathBuf {
    let root = data_root.join("cache");
    match core {
        Some(core) => root.join(core),
        None => root,
    }
}

pub(in crate::commands::cache) fn workspace_build_cache_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    if let Some(root) = find_cargo_workspace_root(&cwd) {
        return Ok(root.join("target"));
    }
    Ok(cwd.join("target"))
}

pub(in crate::commands::cache) fn find_cargo_workspace_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    let mut nearest_manifest = None;

    loop {
        let manifest = current.join("Cargo.toml");
        if manifest.exists() {
            nearest_manifest = Some(current.clone());
            if std::fs::read_to_string(&manifest)
                .map(|contents| contents.contains("[workspace]"))
                .unwrap_or(false)
            {
                return Some(current);
            }
        }

        if !current.pop() {
            break;
        }
    }

    nearest_manifest
}
