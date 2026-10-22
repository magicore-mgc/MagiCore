//! Shared project manifest recognition.
//! Nhận diện manifest dự án shared.
use std::path::{Path, PathBuf};
/// A project signature is only recognized when it is a regular file owned
/// by the project tree; symlinked or special-file signatures fail closed.
/// Chỉ nhận signature là file thường trong project; symlink/file đặc biệt bị từ chối.
pub(in crate::project::project::core_detection) fn signature_file_exists(
    path: &Path,
) -> anyhow::Result<bool> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            anyhow::bail!(
                "project signature '{}' must not be a symlink",
                path.display()
            )
        }
        Ok(metadata) if metadata.is_file() => Ok(true),
        Ok(_) => anyhow::bail!(
            "project signature '{}' must be a regular file",
            path.display()
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

/// Check a nested signature while refusing symlinked parent directories.
/// Kiểm tra signature lồng nhau và từ chối thư mục cha là symlink.
pub(in crate::project::project::core_detection::signatures::signatures) fn signature_file_exists_in_project(
    root: &Path,
    relative: &Path,
) -> anyhow::Result<bool> {
    use std::path::Component;

    let components = relative.components().collect::<Vec<_>>();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        anyhow::bail!("project signature path must be a normalized relative path");
    }

    let mut path = root.to_path_buf();
    for component in components.iter().take(components.len() - 1) {
        path.push(component.as_os_str());
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if metadata.file_type().is_symlink() {
            anyhow::bail!(
                "project signature parent '{}' must not be a symlink",
                path.display()
            );
        }
        if !metadata.is_dir() {
            anyhow::bail!(
                "project signature parent '{}' must be a directory",
                path.display()
            );
        }
    }
    let Some(last) = components.last() else {
        return Ok(false);
    };
    path.push(last.as_os_str());
    signature_file_exists(&path)
}

/// Match known framework coordinates without interpreting version syntax.
/// Nhận diện tọa độ framework phổ biến, không phân tích constraint phiên bản.
pub(in crate::project::project::core_detection::signatures::signatures) fn text_contains_any(
    content: &str,
    markers: &[&str],
) -> bool {
    let lower = content.to_ascii_lowercase();
    markers
        .iter()
        .any(|marker| lower.contains(&marker.to_ascii_lowercase()))
}

/// Check dependency tables, including Cargo workspace and target-specific tables.
/// Kiểm tra dependency table kể cả workspace và cấu hình theo target.
pub(in crate::project::project::core_detection::signatures::signatures) fn has_named_dependency(
    manifest: &toml::Value,
    names: &[&str],
) -> bool {
    fn in_table(value: &toml::Value, names: &[&str]) -> bool {
        let Some(table) = value.as_table() else {
            return false;
        };
        let has_dependency = ["dependencies", "dev-dependencies", "build-dependencies"]
            .iter()
            .filter_map(|section| table.get(*section).and_then(toml::Value::as_table))
            .any(|dependencies| {
                dependencies.iter().any(|(name, spec)| {
                    names.contains(&name.as_str())
                        || spec
                            .get("package")
                            .and_then(toml::Value::as_str)
                            .is_some_and(|package| names.contains(&package))
                })
            });
        has_dependency
            || ["workspace", "target"]
                .iter()
                .filter_map(|section| table.get(*section))
                .any(|nested| in_table(nested, names))
    }

    in_table(manifest, names)
}

/// Find a regular root-level manifest by extension, rejecting a matching symlink.
/// Tìm manifest thường theo extension và từ chối file khớp là symlink.
pub(in crate::project::project::core_detection::signatures::signatures) fn has_regular_extension_file(
    root: &Path,
    extension: &str,
) -> anyhow::Result<bool> {
    Ok(!files_with_extension(root, extension)?.is_empty())
}

/// Collect regular root-level manifests by extension, rejecting matching symlinks.
/// Thu thập manifest thường theo extension và từ chối file khớp là symlink.
pub(in crate::project::project::core_detection::signatures::signatures) fn files_with_extension(
    root: &Path,
    extension: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let entries = match std::fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension))
        {
            continue;
        }
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            anyhow::bail!(
                "project signature '{}' must not be a symlink",
                path.display()
            );
        }
        if file_type.is_file() {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}
