//! Refuse linked roots, parents and targets below the extraction root.
//! Từ chối root, parent và target dạng link bên dưới root giải nén.

use super::super::{ExtractionError, ExtractionResult};
use std::path::{Component, Path, PathBuf};

pub(in crate::extract) fn ensure_extract_root(dest: &Path) -> ExtractionResult<PathBuf> {
    match std::fs::symlink_metadata(dest) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(ExtractionError::RootSymlink(dest.to_path_buf()));
        }
        Ok(metadata) if !metadata.is_dir() => {
            return Err(ExtractionError::RootNotDirectory(dest.to_path_buf()));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir_all(dest)?;
        }
        Err(error) => return Err(error.into()),
    }
    let metadata = std::fs::symlink_metadata(dest)?;
    if metadata.file_type().is_symlink() {
        return Err(ExtractionError::RootSymlink(dest.to_path_buf()));
    }
    if !metadata.is_dir() {
        return Err(ExtractionError::RootNotDirectory(dest.to_path_buf()));
    }
    Ok(dest.canonicalize()?)
}

pub(in crate::extract) fn ensure_extract_file_target(
    root: &Path,
    relative: &Path,
) -> ExtractionResult<PathBuf> {
    let components = safe_components(relative)?;
    let (filename, parents) = components
        .split_last()
        .ok_or(ExtractionError::EmptyArchivePath)?;
    let mut current = root.to_path_buf();
    for parent in parents {
        current.push(parent);
        ensure_directory_component(&current)?;
    }
    current.push(filename);
    match std::fs::symlink_metadata(&current) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(ExtractionError::TargetSymlink(current));
        }
        Ok(metadata) if !metadata.is_file() => {
            return Err(ExtractionError::TargetNotFile(current));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    if !current.starts_with(root) {
        return Err(ExtractionError::PathEscapes(current));
    }
    Ok(current)
}

pub(in crate::extract) fn ensure_extract_directory(
    root: &Path,
    relative: &Path,
) -> ExtractionResult<PathBuf> {
    let components = safe_components(relative)?;
    let mut current = root.to_path_buf();
    for component in components {
        current.push(component);
        ensure_directory_component(&current)?;
    }
    if !current.starts_with(root) {
        return Err(ExtractionError::PathEscapes(current));
    }
    Ok(current)
}

fn safe_components(relative: &Path) -> ExtractionResult<Vec<std::ffi::OsString>> {
    relative
        .components()
        .map(|component| match component {
            Component::Normal(part) => Ok(part.to_os_string()),
            _ => Err(ExtractionError::UnsafeArchivePath(relative.to_path_buf())),
        })
        .collect()
}

fn ensure_directory_component(path: &Path) -> ExtractionResult<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(ExtractionError::ParentSymlink(path.to_path_buf()))
        }
        Ok(metadata) if !metadata.is_dir() => {
            Err(ExtractionError::ParentNotDirectory(path.to_path_buf()))
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
