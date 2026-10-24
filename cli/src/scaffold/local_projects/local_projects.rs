//! Local project discovery for offline scaffolds.
//! Dò project local cho scaffold offline.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// A previously scaffolded project that matches the requested core+framework.
/// Project đã scaffold trước đó khớp core+framework yêu cầu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProject {
    pub root: PathBuf,
    pub ecosystem: String,
    pub frameworks: Vec<String>,
}

/// Framework markers that must exist for a project to be usable as a source.
/// Marker framework phải tồn tại để project dùng được làm nguồn.
fn has_framework_marker(root: &Path, ecosystem: &str, framework: &str) -> bool {
    // Shared with the static framework directory so both sides can never
    // disagree about what proves an identity.
    // Dùng chung với danh mục framework tĩnh để hai phía không bao giờ lệch
    // nhau về cái gì chứng minh danh tính.
    match crate::scaffold::framework_registry::framework_marker(ecosystem, framework) {
        Some(marker) => marker.iter().all(|segment| root.join(segment).exists()),
        None => true,
    }
}

/// Discover local projects under `projects_root` matching `ecosystem` +
/// `framework`. Identity comes from `mgc.toml`, never from the folder name.
/// Dò project dưới `projects_root` khớp `ecosystem` + `framework`. Nhận diện
/// bằng `mgc.toml`, không bao giờ bằng tên thư mục.
pub fn discover_local_projects(
    projects_root: &Path,
    ecosystem: &str,
    framework: &str,
) -> Result<Vec<LocalProject>> {
    if !projects_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut matches = Vec::new();
    for entry in std::fs::read_dir(projects_root)?.flatten() {
        let root = entry.path();
        let Ok(metadata) = std::fs::symlink_metadata(&root) else {
            continue;
        };
        // Never follow a symlinked project root: it can escape the data dir.
        // Không bao giờ đi theo project root là symlink: có thể thoát data dir.
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            continue;
        }
        let Ok(Some(config)) = mgc_config::project::ProjectConfig::load(&root) else {
            continue;
        };
        if config.ecosystem != ecosystem {
            continue;
        }
        if !config.frameworks.iter().any(|name| name == framework) {
            continue;
        }
        if !has_framework_marker(&root, ecosystem, framework) {
            continue;
        }
        matches.push(LocalProject {
            root,
            ecosystem: config.ecosystem,
            frameworks: config.frameworks,
        });
    }
    matches.sort_by(|a, b| a.root.cmp(&b.root));
    Ok(matches)
}

/// Copy a discovered project into `target`, excluding generated/vendored
/// trees, `.git` and `.env` (secrets must never travel between projects).
/// Sao chép project tìm được vào `target`, loại cây sinh/vendored, `.git` và
/// `.env` (secret không bao giờ đi giữa các project).
pub fn copy_local_project(source: &Path, target: &Path) -> Result<()> {
    const SKIP: &[&str] = &[
        "node_modules",
        ".venv",
        "venv",
        "target",
        "dist",
        "build",
        ".git",
        ".env",
        ".magicore",
    ];
    copy_filtered(source, target, SKIP)
}

fn copy_filtered(source: &Path, target: &Path, skip: &[&str]) -> Result<()> {
    std::fs::create_dir_all(target)?;
    for entry in std::fs::read_dir(source)?.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if skip.iter().any(|skip| name_str == *skip) {
            continue;
        }
        let from = entry.path();
        let to = target.join(&name);
        let Ok(metadata) = std::fs::symlink_metadata(&from) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            // Copy link targets only when they stay inside the project.
            // Chỉ copy đích link khi nằm trong project.
            let Ok(resolved) = std::fs::canonicalize(&from) else {
                continue;
            };
            let Ok(source_root) = std::fs::canonicalize(source) else {
                continue;
            };
            if !resolved.starts_with(&source_root) {
                continue;
            }
            if resolved.is_dir() {
                copy_filtered(&from, &to, skip)?;
            } else {
                std::fs::copy(&from, &to)?;
            }
            continue;
        }
        if metadata.is_dir() {
            copy_filtered(&from, &to, skip)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
