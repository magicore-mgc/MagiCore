//! Confines cache operations to the selected root and rejects symlink entries.
//! Giới hạn thao tác trong cache root và từ chối entry symlink.

use crate::package_cache::error::CacheError;
use anyhow::Result;
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path};

pub(in crate::package_cache) fn ensure_cache_directory(
    root: &Path,
    directory: &Path,
) -> Result<()> {
    fs::create_dir_all(root)?;
    let relative = relative_to_root(root, directory)?;
    let canonical_root = fs::canonicalize(root)?;
    let mut current = canonical_root;

    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(CacheError::UnsafeCachePath(directory.to_path_buf()).into());
        };
        current.push(name);
        match fs::symlink_metadata(&current) {
            Ok(metadata) => validate_directory_entry(&metadata, directory)?,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(create_error) if create_error.kind() == ErrorKind::AlreadyExists => {}
                    Err(create_error) => return Err(create_error.into()),
                }
                let metadata = fs::symlink_metadata(&current)?;
                validate_directory_entry(&metadata, directory)?;
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

pub(in crate::package_cache) fn ensure_parent_within_root(root: &Path, path: &Path) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(CacheError::UnsafeCachePath(path.to_path_buf()).into());
    };
    validate_cache_directory(root, parent, path)
}

pub(in crate::package_cache) fn is_regular_cache_file(root: &Path, path: &Path) -> Result<bool> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(CacheError::UnsafeCachePath(path.to_path_buf()).into());
    }
    ensure_parent_within_root(root, path)?;
    Ok(true)
}

pub(in crate::package_cache) fn remove_path_no_follow(root: &Path, path: &Path) -> Result<bool> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    ensure_parent_within_root(root, path)?;

    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        fs::remove_file(path)?;
    } else {
        validate_cache_directory(root, path, path)?;
        fs::remove_dir_all(path)?;
    }
    Ok(true)
}

fn validate_cache_directory(root: &Path, directory: &Path, reported_path: &Path) -> Result<()> {
    let relative = relative_to_root(root, directory)
        .map_err(|_| CacheError::UnsafeCachePath(reported_path.to_path_buf()))?;
    let canonical_root = fs::canonicalize(root)?;
    let mut current = canonical_root;
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(CacheError::UnsafeCachePath(reported_path.to_path_buf()).into());
        };
        current.push(name);
        let metadata = fs::symlink_metadata(&current)?;
        validate_directory_entry(&metadata, reported_path)?;
    }
    Ok(())
}

fn relative_to_root<'a>(root: &Path, candidate: &'a Path) -> Result<&'a Path> {
    candidate
        .strip_prefix(root)
        .map_err(|_| CacheError::UnsafeCachePath(candidate.to_path_buf()).into())
}

fn validate_directory_entry(metadata: &fs::Metadata, reported_path: &Path) -> Result<()> {
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(CacheError::UnsafeCachePath(reported_path.to_path_buf()).into());
    }
    Ok(())
}
