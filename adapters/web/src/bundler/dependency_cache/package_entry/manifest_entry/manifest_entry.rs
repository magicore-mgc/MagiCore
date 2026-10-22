//! Resolve package.json fields within the canonical package root.
//! Resolve field package.json bên trong canonical package root.

use std::path::{Component, Path, PathBuf};

use super::super::{PackageEntryError, resolve_file_inside_package};

/// Select the first available browser/module/main entry or `index.js`.
/// Chọn entry browser/module/main đầu tiên có file, nếu không dùng `index.js`.
pub(in crate::bundler::dependency_cache::package_entry::implementation) fn resolve_manifest_entry(
    package_root: &Path,
    selected_directory: &Path,
) -> Result<Option<PathBuf>, PackageEntryError> {
    let manifest_path = match resolve_file_inside_package(
        package_root,
        &selected_directory.join("package.json"),
    )? {
        Some(path) => path,
        None => return fallback_index(package_root, selected_directory),
    };

    if let Ok(raw) = std::fs::read_to_string(manifest_path)
        && let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&raw)
    {
        for field in ["browser", "module", "main"] {
            let Some(entry) = manifest.get(field).and_then(serde_json::Value::as_str) else {
                continue;
            };
            let relative = normalize_manifest_entry(entry)?;
            if let Some(path) =
                resolve_file_inside_package(package_root, &selected_directory.join(relative))?
            {
                return Ok(Some(path));
            }
        }
    }

    fallback_index(package_root, selected_directory)
}

fn fallback_index(
    package_root: &Path,
    selected_directory: &Path,
) -> Result<Option<PathBuf>, PackageEntryError> {
    resolve_file_inside_package(package_root, &selected_directory.join("index.js"))
}

fn normalize_manifest_entry(entry: &str) -> Result<PathBuf, PackageEntryError> {
    if entry.is_empty()
        || entry.contains('\\')
        || entry.contains(':')
        || entry.chars().any(|character| character.is_control())
    {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let path = Path::new(entry);
    if path.is_absolute() {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(value) => normalized.push(value),
            Component::ParentDir => normalized.push(".."),
            Component::RootDir | Component::Prefix(_) => {
                return Err(PackageEntryError::ForbiddenPath);
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(PackageEntryError::ForbiddenPath);
    }
    Ok(normalized)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
