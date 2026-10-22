//! Finds the one explicitly MAUI-enabled project file.
//! Tìm file project duy nhất khai báo bật MAUI.

use std::path::{Path, PathBuf};

use super::super::is_regular_manifest;

/// Returns a project only when exactly one regular `.csproj` enables MAUI.
/// Chỉ trả project khi đúng một `.csproj` thường bật MAUI.
pub(crate) fn find(root: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(root).ok()?;
    let mut projects = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "csproj")
                && is_regular_manifest(path)
                && mgc_config::project::read_regular_project_text(path, "MAUI project")
                    .ok()
                    .flatten()
                    .is_some_and(|contents| contents.contains("<UseMaui>true</UseMaui>"))
        })
        .collect::<Vec<_>>();
    projects.sort();
    (projects.len() == 1).then(|| projects.remove(0))
}
