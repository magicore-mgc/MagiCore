//! Measure logical bytes without following links — đo byte file thường và bỏ qua symlink.

use super::error::{LogicalSizeError, LogicalSizeResult};
use std::fs;
use std::path::Path;

pub(in crate::cache) fn logical_size(path: &Path) -> LogicalSizeResult<u64> {
    let mut pending = vec![path.to_path_buf()];
    let mut total = 0u64;

    while let Some(current) = pending.pop() {
        let metadata = fs::symlink_metadata(&current)
            .map_err(|source| LogicalSizeError::io("inspect cache entry", source))?;
        let file_type = metadata.file_type();

        if file_type.is_file() {
            total = total.saturating_add(metadata.len());
            continue;
        }

        if !file_type.is_dir() {
            continue;
        }

        let entries = fs::read_dir(&current)
            .map_err(|source| LogicalSizeError::io("read cache directory", source))?;
        for entry in entries {
            let entry = entry
                .map_err(|source| LogicalSizeError::io("read cache directory entry", source))?;
            let file_type = entry
                .file_type()
                .map_err(|source| LogicalSizeError::io("inspect cache directory entry", source))?;

            if file_type.is_symlink() {
                continue;
            }

            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                let metadata = fs::symlink_metadata(entry.path())
                    .map_err(|source| LogicalSizeError::io("measure cache file", source))?;
                if metadata.file_type().is_file() {
                    total = total.saturating_add(metadata.len());
                }
            }
        }
    }

    Ok(total)
}
