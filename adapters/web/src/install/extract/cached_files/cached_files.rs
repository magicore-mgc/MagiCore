//! Enumerate cached package files and compare them against archive bytes.
//! Liệt kê file package cache và so sánh với byte archive.
use super::{
    file_identity::{cached_file_identity, open_cached_file_no_follow},
    types::CachedFileMetadata,
};
use crate::install::package_marker::{extracted_package_marker_path, metadata_is_executable};
use mgc_types::{MgError, MgResult};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

pub(crate) fn cached_archive_file_matches(
    package_root: &Path,
    relative_path: &Path,
    cached_metadata: CachedFileMetadata,
    expected_data: &[u8],
    expected_executable: bool,
) -> MgResult<bool> {
    if cached_metadata.len != expected_data.len() as u64
        || cached_metadata.executable != expected_executable
    {
        return Ok(false);
    }
    let target = package_root.join(relative_path);
    let Some(mut file) =
        open_cached_file_no_follow(package_root, relative_path).map_err(|err| {
            MgError::Other(format!(
                "failed to open cached package file '{}': {err}",
                target.display()
            ))
        })?
    else {
        return Ok(false);
    };
    let opened_metadata = file.metadata().map_err(|err| {
        MgError::Other(format!(
            "failed to inspect opened cached package file '{}': {err}",
            target.display()
        ))
    })?;
    if cached_file_identity(&file, &opened_metadata) != Some(cached_metadata.identity)
        || opened_metadata.len() != cached_metadata.len
        || metadata_is_executable(&opened_metadata) != cached_metadata.executable
    {
        return Ok(false);
    }
    let mut offset = 0usize;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|err| {
            MgError::Other(format!(
                "failed to compare cached package file '{}': {err}",
                target.display()
            ))
        })?;
        if read == 0 {
            return Ok(offset == expected_data.len());
        }
        let Some(end) = offset.checked_add(read) else {
            return Ok(false);
        };
        if end > expected_data.len() || buffer[..read] != expected_data[offset..end] {
            return Ok(false);
        }
        offset = end;
    }
}

pub(crate) fn cached_package_files(
    root: &Path,
) -> MgResult<Option<HashMap<PathBuf, CachedFileMetadata>>> {
    let mut files = HashMap::new();
    for entry in walkdir::WalkDir::new(root).min_depth(1) {
        let entry = entry.map_err(|err| {
            MgError::Other(format!(
                "failed to walk cached package '{}': {err}",
                root.display()
            ))
        })?;
        if entry.path() == extracted_package_marker_path(root) {
            continue;
        }
        if entry.file_type().is_dir() {
            continue;
        }
        if !entry.file_type().is_file() {
            return Ok(None);
        }
        let relative = entry.path().strip_prefix(root).map_err(|err| {
            MgError::Other(format!(
                "failed to inspect cached package path '{}': {err}",
                entry.path().display()
            ))
        })?;
        let Some(file) = open_cached_file_no_follow(root, relative).map_err(|err| {
            MgError::Other(format!(
                "failed to securely open cached package file '{}': {err}",
                entry.path().display()
            ))
        })?
        else {
            return Ok(None);
        };
        let metadata = file.metadata().map_err(|err| {
            MgError::Other(format!(
                "failed to inspect opened cached package file '{}': {err}",
                entry.path().display()
            ))
        })?;
        let Some(identity) = cached_file_identity(&file, &metadata) else {
            return Ok(None);
        };
        files.insert(
            relative.to_path_buf(),
            CachedFileMetadata {
                len: metadata.len(),
                executable: metadata_is_executable(&metadata),
                identity,
            },
        );
    }
    Ok(Some(files))
}
