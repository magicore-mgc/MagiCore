//! Normalize archive paths and reject unsafe filesystem components.
//! Chuẩn hóa path archive và chặn thành phần filesystem không an toàn.

use super::super::{ArchiveError, ArchiveResult};
use std::path::{Component, Path, PathBuf};

pub(in crate::protocols::archive) fn split_safe_relative(
    relative: &Path,
) -> ArchiveResult<(Vec<std::ffi::OsString>, std::ffi::OsString)> {
    let components: Vec<_> = relative
        .components()
        .map(|component| match component {
            Component::Normal(part) => Ok(part.to_os_string()),
            _ => Err(ArchiveError::Integrity(format!(
                "unsafe archive path component: {}",
                relative.display()
            ))),
        })
        .collect::<ArchiveResult<_>>()?;
    let (filename, parent) = components
        .split_last()
        .ok_or_else(|| ArchiveError::Integrity("empty archive relative path".to_string()))?;
    Ok((parent.to_vec(), filename.clone()))
}

pub(in crate::protocols::archive) fn ensure_directory_component(path: &Path) -> ArchiveResult<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(ArchiveError::Integrity(format!(
                "refusing linked or non-directory archive parent: {}",
                path.display()
            )))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match std::fs::create_dir(path) {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    ensure_directory_component(path)
                }
                Err(error) => Err(error.into()),
            }
        }
        Err(error) => Err(error.into()),
    }
}

/// Reject any path component that would escape the destination root.
/// Từ chối mọi thành phần path có thể thoát khỏi gốc đích.
pub(in crate::protocols::archive) fn sanitize_archive_path(path: &Path) -> ArchiveResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ArchiveError::Other(format!(
                    "unsafe tar entry path: {}",
                    path.display()
                )));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(ArchiveError::Other("empty tar entry path".to_string()));
    }
    Ok(clean)
}
