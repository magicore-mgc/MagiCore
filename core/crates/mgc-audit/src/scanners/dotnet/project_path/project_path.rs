//! Resolve `.sln` project paths without allowing root escapes.
//! Resolve path project từ `.sln` nhưng không cho phép thoát root.

use std::path::{Component, Path, PathBuf};

/// Canonical project root for escape checks.
/// Root canonical của project để kiểm tra path thoát ra ngoài.
pub(in crate::scanners::dotnet) fn canonical_root(project_root: &Path) -> PathBuf {
    project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf())
}

/// Resolve case drift only when a path component has one unique match.
/// Chỉ resolve path lệch hoa thường khi mỗi component có đúng một kết quả.
pub(in crate::scanners::dotnet) fn resolve_project_path(
    project_root: &Path,
    relative_path: &str,
) -> Option<PathBuf> {
    let direct = project_root.join(relative_path);
    if direct.is_file() {
        return Some(direct);
    }
    let mut current = project_root.to_path_buf();
    for component in Path::new(relative_path).components() {
        let wanted = match component {
            Component::Normal(part) => part.to_string_lossy().to_lowercase(),
            _ => return None,
        };
        let mut matches = std::fs::read_dir(&current)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().to_lowercase() == wanted);
        let hit = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        current = hit.path();
    }
    current.is_file().then_some(current)
}
