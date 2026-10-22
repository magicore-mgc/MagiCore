//! Recursive project target scan and manifest recognition.
//! Quét target project đệ quy và nhận diện manifest.

use super::super::layout_path::workspace_entry_is_directory;
use std::path::{Path, PathBuf};

/// Backend manifest names supported by the current discovery contract.
/// Tên backend manifest được hỗ trợ theo hợp đồng discovery hiện tại.
const BACKEND_MANIFEST_NAMES: &[&str] = &[
    // Native project manifests — manifest project native.
    "mgc.toml",
    "pyproject.toml",
    "west.yml",
    "platformio.ini",
    "project.godot",
    // Existing web/backend manifests — manifest web/backend hiện có.
    "go.mod",
    "Cargo.toml",
    "manage.py",
    "main.py",
    "src/main.py",
    "pom.xml",
    "artisan",
    "composer.json",
];

/// Collect manifest-bearing workspace projects recursively.
/// Thu thập đệ quy project workspace có manifest.
pub(in crate::discover) fn collect_projects(
    root: PathBuf,
    out: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    if !workspace_entry_is_directory(&root)? {
        return Ok(());
    }
    for entry in std::fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();
        if !workspace_entry_is_directory(&path)? {
            continue;
        }
        if path.join("package.json").exists() || has_backend_manifest(&path) {
            out.push(path);
            continue;
        }
        collect_projects(path, out)?;
    }
    Ok(())
}

/// Detect a backend project using the established native and web manifest names.
/// Nhận diện backend theo danh sách manifest native và web hiện có.
pub fn has_backend_manifest(path: &Path) -> bool {
    BACKEND_MANIFEST_NAMES
        .iter()
        .any(|name| path.join(name).exists())
}
