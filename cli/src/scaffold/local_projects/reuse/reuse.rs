//! Reuse a project already on the user's machine as the scaffold source.
//! Tái dùng project đã có trên máy user làm nguồn scaffold.
//!
//! Only framework source is copied. MagiCore-owned artifacts (`mgc.toml`,
//! lockfile, `.magicore/`) are written by the caller, never carried over.
//! Chỉ copy source framework. Artifact do MagiCore sở hữu (`mgc.toml`,
//! lockfile, `.magicore/`) do caller ghi, không mang theo.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::scaffold::local_projects::discover_local_projects;

/// Copy `source` project files into `target` and report the source used.
/// Sao chép file project `source` vào `target` và trả về nguồn đã dùng.
pub fn reuse_local_project(source: &Path, target: &Path, frameworks: &[String]) -> Result<PathBuf> {
    copy_source_only(source, target)
        .with_context(|| format!("reuse local project {}", source.display()))?;
    // The copied manifest is the old project's; the caller writes a new one
    // so identity (name/version) cannot leak between projects.
    // Manifest copy được là của project cũ; caller ghi manifest mới để
    // identity (name/version) không rỉ giữa các project.
    let stale_manifest = target.join("mgc.toml");
    if stale_manifest.exists() {
        std::fs::remove_file(&stale_manifest)
            .context("remove copied mgc.toml before writing the new identity")?;
    }
    for lockfile in [
        "mgc.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
        "Cargo.lock",
    ] {
        let stale = target.join(lockfile);
        if stale.exists() {
            std::fs::remove_file(&stale).ok();
        }
    }
    let _ = frameworks;
    Ok(source.to_path_buf())
}

fn copy_source_only(source: &Path, target: &Path) -> Result<()> {
    const MAGICORE_OWNED: &[&str] = &["mgc.toml", "mgc.lock", ".magicore", ".mgc.core"];
    std::fs::create_dir_all(target)?;
    for entry in std::fs::read_dir(source)?.flatten() {
        let name = entry.file_name();
        if MAGICORE_OWNED
            .iter()
            .any(|owned| name.to_string_lossy() == *owned)
        {
            continue;
        }
        let from = entry.path();
        let to = target.join(&name);
        let Ok(metadata) = std::fs::symlink_metadata(&from) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            let (Ok(resolved), Ok(root)) =
                (std::fs::canonicalize(&from), std::fs::canonicalize(source))
            else {
                continue;
            };
            if !resolved.starts_with(&root) {
                continue;
            }
            if resolved.is_dir() {
                copy_filtered(&from, &to, MAGICORE_OWNED)?;
            } else {
                std::fs::copy(&from, &to)?;
            }
            continue;
        }
        if metadata.is_dir() {
            copy_filtered(&from, &to, MAGICORE_OWNED)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn copy_filtered(source: &Path, target: &Path, skip_named: &[&str]) -> Result<()> {
    const SKIP: &[&str] = &[
        "node_modules",
        ".venv",
        "venv",
        "target",
        "dist",
        "build",
        ".git",
        ".env",
    ];
    std::fs::create_dir_all(target)?;
    for entry in std::fs::read_dir(source)?.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if SKIP.iter().chain(skip_named).any(|skip| name_str == *skip) {
            continue;
        }
        let from = entry.path();
        let to = target.join(&name);
        let Ok(metadata) = std::fs::symlink_metadata(&from) else {
            continue;
        };
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            copy_filtered(&from, &to, skip_named)?;
        } else if !metadata.file_type().is_symlink() {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Resolve the offline source for a scaffold config: an explicit
/// `--from-local` path, or an automatic match under the MagiCore data dir.
/// Returns `None` when `--no-local` disables the branch or nothing matched;
/// the caller must then fall back to the registry layer or fail closed.
/// Resolve nguồn offline cho config scaffold: đường `--from-local` tường minh,
/// hoặc project khớp tự động dưới data dir MagiCore. Trả `None` khi
/// `--no-local` tắt nhánh hoặc không khớp gì; caller khi đó phải dùng registry
/// layer hoặc fail-closed.
pub fn resolve_offline_source(
    config: &crate::wizard::engine::ScaffoldConfig,
    framework: &str,
) -> Result<Option<PathBuf>> {
    if let Some(explicit) = config.from_local.as_deref() {
        return Ok(Some(explicit.to_path_buf()));
    }
    if config.no_local {
        return Ok(None);
    }
    let projects_root = mgc_platform::paths::mgc_data_dir().join("projects");
    let ecosystem = crate::scaffold::framework_registry::ecosystem_for_core(&config.core);
    find_reusable(&projects_root, ecosystem, framework)
}

/// Find a reusable local project for this core/framework, if any.
/// Tìm project local có thể tái dùng cho core/framework này.
pub fn find_reusable(
    projects_root: &Path,
    ecosystem: &str,
    framework: &str,
) -> Result<Option<PathBuf>> {
    let matches = discover_local_projects(projects_root, ecosystem, framework)?;
    Ok(matches.first().map(|project| project.root.clone()))
}
