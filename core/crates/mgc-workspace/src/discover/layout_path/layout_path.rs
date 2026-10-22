//! Workspace path validation and non-following directory checks.
//! Kiểm tra đường dẫn và thư mục workspace mà không đi theo symlink.

use std::path::{Component, Path, PathBuf};

/// Resolve a workspace layout entry as a project-local, non-symlink directory.
/// Chỉ phân giải layout tương đối trong project và từ chối mọi symlink.
pub fn resolve_workspace_layout_dir(
    project_root: &Path,
    configured_path: &str,
    field_name: &str,
) -> anyhow::Result<PathBuf> {
    let relative = Path::new(configured_path);
    if relative.as_os_str().is_empty()
        || relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        anyhow::bail!(
            "workspace layout path '{field_name}' must be a non-empty relative path without parent or root components"
        );
    }

    let mut candidate = project_root.to_path_buf();
    for component in relative.components() {
        candidate.push(component.as_os_str());
        match std::fs::symlink_metadata(&candidate) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                anyhow::bail!(
                    "workspace layout path '{field_name}' must not traverse symlink '{}'",
                    candidate.display()
                );
            }
            Ok(metadata) if !metadata.is_dir() => return Ok(candidate),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(candidate),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(candidate)
}

/// Check whether a path is a directory without following its final symlink.
/// Kiểm tra thư mục mà không đi theo symlink ở thành phần cuối.
pub fn workspace_entry_is_directory(path: &Path) -> anyhow::Result<bool> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            anyhow::bail!(
                "workspace directory '{}' must not be a symlink",
                path.display()
            );
        }
        Ok(metadata) => Ok(metadata.is_dir()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}
