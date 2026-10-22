//! Fail-closed removal of stale content-addressed blobs.
//! Xóa blob content-addressed cũ theo nguyên tắc fail-closed.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Result;

use crate::database::Database;

use super::super::error::IndexError;
use super::super::types::FileEntry;

pub(in crate::index) fn prune_blobs(
    db: &Database,
    packages: &HashMap<String, Vec<FileEntry>>,
    cas_root: &Path,
    max_age: std::time::Duration,
) -> Result<usize> {
    if !cas_root.exists() {
        return Ok(0);
    }
    let live: HashSet<String> = db.list_all_blob_hashes()?.into_iter().collect();
    let indexed_count = db.count_indexed_packages()?;
    if indexed_count == 0 && !packages.is_empty() {
        return Err(IndexError::EmptySqliteDuringPrune.into());
    }
    if live.is_empty() {
        return Ok(0);
    }

    let mut removed = 0usize;
    let blobs_root = cas_root.join("files").join("blake3");
    if !blobs_root.exists() {
        return Ok(0);
    }
    for entry in walkdir::WalkDir::new(&blobs_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() || !path_is_older_than(entry.path(), max_age) {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        let hash = name.trim_end_matches(".exec").to_string();
        if live.contains(&hash) {
            continue;
        }
        if !file_has_no_external_hardlinks(entry.path()) {
            continue;
        }
        if std::fs::remove_file(entry.path()).is_ok() {
            removed += 1;
        }
    }
    Ok(removed)
}

fn path_is_older_than(path: &Path, max_age: std::time::Duration) -> bool {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map(|modified| modified.elapsed().map(|age| age > max_age).unwrap_or(false))
        .unwrap_or(false)
}

fn file_has_no_external_hardlinks(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        std::fs::metadata(path)
            .map(|metadata| metadata.nlink() <= 1)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        false
    }
}
