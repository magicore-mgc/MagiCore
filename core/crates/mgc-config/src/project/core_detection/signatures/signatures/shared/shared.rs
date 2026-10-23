//! Shared project manifest recognition.
//! Nhận diện manifest dự án shared.

use std::path::Path;

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

#[path = "content_probe/mod.rs"]
mod content_probe;

pub(in crate::project::project::core_detection::signatures::signatures) use content_probe::{
    files_with_extension, has_named_dependency, has_regular_extension_file, text_contains_any,
};
