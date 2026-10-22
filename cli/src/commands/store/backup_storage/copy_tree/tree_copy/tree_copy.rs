//! Streams real entries first and publishes validated package links in a second pass.
//! Stream entry thật trước rồi publish package link đã xác thực ở lượt hai.

use std::fs;
use std::path::{Component, Path, PathBuf};

use super::super::destination::ensure_destination_directory;
use super::super::source_validation::{validate_real_directory, validated_link_target};
use super::error::TreeCopyError;
use crate::commands::safe_filesystem::copy_regular_file_no_follow;

pub(in crate::commands::store::backup_storage::copy_tree) fn copy_directory(
    source_root: &Path,
    destination_root: &Path,
    source_directory: &Path,
    destination_directory: &Path,
) -> Result<(), TreeCopyError> {
    validate_real_directory(source_directory, source_root)?;
    ensure_destination_directory(destination_directory, destination_root)?;
    let entries =
        fs::read_dir(source_directory).map_err(|source| TreeCopyError::ReadDirectory {
            path: source_directory.to_path_buf(),
            source,
        })?;

    for entry in entries {
        let entry = entry.map_err(|source| TreeCopyError::ReadEntry {
            path: source_directory.to_path_buf(),
            source,
        })?;
        let source_path = entry.path();
        let destination_path = destination_directory.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|source| TreeCopyError::InspectEntry {
                path: source_path.clone(),
                source,
            })?;
        if file_type.is_symlink() {
            continue;
        } else if file_type.is_dir() {
            ensure_destination_directory(&destination_path, destination_root)?;
            copy_directory(
                source_root,
                destination_root,
                &source_path,
                &destination_path,
            )?;
        } else if file_type.is_file() {
            copy_regular_file_no_follow(&source_path, &destination_path)?;
        } else {
            return Err(TreeCopyError::SpecialEntry(source_path));
        }
    }
    Ok(())
}

pub(in crate::commands::store::backup_storage::copy_tree) fn copy_links(
    source_root: &Path,
    destination_root: &Path,
    source_directory: &Path,
    destination_directory: &Path,
) -> Result<(), TreeCopyError> {
    validate_real_directory(source_directory, source_root)?;
    ensure_destination_directory(destination_directory, destination_root)?;
    let entries =
        fs::read_dir(source_directory).map_err(|source| TreeCopyError::ReadDirectory {
            path: source_directory.to_path_buf(),
            source,
        })?;
    for entry in entries {
        let entry = entry.map_err(|source| TreeCopyError::ReadEntry {
            path: source_directory.to_path_buf(),
            source,
        })?;
        let source_path = entry.path();
        let destination_path = destination_directory.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|source| TreeCopyError::InspectEntry {
                path: source_path.clone(),
                source,
            })?;
        if file_type.is_symlink() {
            copy_internal_link(
                source_root,
                destination_root,
                &source_path,
                &destination_path,
            )?;
        } else if file_type.is_dir() {
            copy_links(
                source_root,
                destination_root,
                &source_path,
                &destination_path,
            )?;
        } else if !file_type.is_file() {
            return Err(TreeCopyError::SpecialEntry(source_path));
        }
    }
    Ok(())
}

pub(in crate::commands::store::backup_storage::copy_tree) fn copy_internal_link(
    source_root: &Path,
    destination_root: &Path,
    source_link: &Path,
    destination_link: &Path,
) -> Result<(), TreeCopyError> {
    let (target_suffix, is_directory) = validated_link_target(source_root, source_link)?;
    let destination_target = destination_root.join(target_suffix);
    let destination_parent =
        destination_link
            .parent()
            .ok_or_else(|| TreeCopyError::RebaseLink {
                link: source_link.to_path_buf(),
                reason: "destination link has no parent",
            })?;
    let relative_target =
        relative_path(destination_parent, &destination_target).ok_or_else(|| {
            TreeCopyError::RebaseLink {
                link: source_link.to_path_buf(),
                reason: "source and destination targets do not share a path root",
            }
        })?;

    match fs::symlink_metadata(destination_link) {
        Ok(metadata)
            if metadata.file_type().is_symlink()
                && fs::read_link(destination_link).ok().as_ref() == Some(&relative_target) =>
        {
            return Ok(());
        }
        Ok(_) => {
            return Err(TreeCopyError::LinkDestinationConflict(
                destination_link.to_path_buf(),
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(TreeCopyError::InspectEntry {
                path: destination_link.to_path_buf(),
                source,
            });
        }
    }
    create_symlink(&relative_target, destination_link, is_directory).map_err(|source| {
        TreeCopyError::CreateLink {
            path: destination_link.to_path_buf(),
            source,
        }
    })
}

fn relative_path(from: &Path, to: &Path) -> Option<PathBuf> {
    let from_components: Vec<_> = from.components().collect();
    let to_components: Vec<_> = to.components().collect();
    let common = from_components
        .iter()
        .zip(&to_components)
        .take_while(|(left, right)| left == right)
        .count();
    if common == 0 {
        return None;
    }
    let mut relative = PathBuf::new();
    for component in &from_components[common..] {
        if matches!(component, Component::Normal(_)) {
            relative.push("..");
        } else {
            return None;
        }
    }
    for component in &to_components[common..] {
        relative.push(component.as_os_str());
    }
    if relative.as_os_str().is_empty() {
        relative.push(".");
    }
    Some(relative)
}

fn create_symlink(target: &Path, link: &Path, is_directory: bool) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let _ = is_directory;
        std::os::unix::fs::symlink(target, link)
    }
    #[cfg(windows)]
    {
        if is_directory {
            std::os::windows::fs::symlink_dir(target, link)
        } else {
            std::os::windows::fs::symlink_file(target, link)
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (target, link, is_directory);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            TreeCopyError::UnsupportedPlatform,
        ))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
