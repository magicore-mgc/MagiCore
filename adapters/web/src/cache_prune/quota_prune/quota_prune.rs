//! Collect and remove cache entries to meet the configured quota.
//! Thu thập và xóa cache entry để đáp ứng quota đã cấu hình.

use std::path::{Path, PathBuf};

use mgc_types::MgResult;
use walkdir::WalkDir;

use crate::cache_prune::{canonical_or_original, cleanup_empty_dirs};

#[derive(Debug, Clone, Copy)]
pub enum CachePruneEntryKind {
    File,
    Directory,
}

#[derive(Debug)]
pub struct CachePruneEntry {
    pub path: PathBuf,
    pub bytes: u64,
    pub modified: std::time::SystemTime,
    pub kind: CachePruneEntryKind,
}

pub fn prune_shared_cache_to_quota(
    root: &Path,
    max_bytes: u64,
    pinned_package_roots: &std::collections::HashSet<PathBuf>,
) -> MgResult<()> {
    if max_bytes == 0 || !root.exists() {
        return Ok(());
    }

    let mut entries = Vec::new();
    collect_prunable_files(&root.join("cache"), &mut entries);
    collect_prunable_files(&root.join("metadata"), &mut entries);
    collect_prunable_files(&root.join("resolutions"), &mut entries);
    collect_prunable_package_dirs(&root.join("packages"), &mut entries, pinned_package_roots);

    let mut total: u64 = entries.iter().map(|entry| entry.bytes).sum();
    if total <= max_bytes {
        return Ok(());
    }

    entries.sort_by_key(|entry| entry.modified);
    for entry in entries {
        if total <= max_bytes {
            break;
        }
        let removed = match entry.kind {
            CachePruneEntryKind::File => std::fs::remove_file(&entry.path).is_ok(),
            CachePruneEntryKind::Directory => std::fs::remove_dir_all(&entry.path).is_ok(),
        };
        if removed {
            total = total.saturating_sub(entry.bytes);
        }
    }

    cleanup_empty_dirs(&root.join("cache"));
    cleanup_empty_dirs(&root.join("metadata"));
    cleanup_empty_dirs(&root.join("resolutions"));
    cleanup_empty_dirs(&root.join("packages"));
    Ok(())
}

pub fn collect_prunable_files(root: &Path, entries: &mut Vec<CachePruneEntry>) {
    if !root.exists() {
        return;
    }
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        entries.push(CachePruneEntry {
            path: entry.path().to_path_buf(),
            bytes: metadata.len(),
            modified: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
            kind: CachePruneEntryKind::File,
        });
    }
}

pub fn collect_prunable_package_dirs(
    root: &Path,
    entries: &mut Vec<CachePruneEntry>,
    pinned_package_roots: &std::collections::HashSet<PathBuf>,
) {
    if !root.exists() {
        return;
    }
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        if pinned_package_roots.contains(&canonical_or_original(path)) {
            continue;
        }
        let marker = path.join(".magicore-package-root.json");
        if !marker.exists() {
            continue;
        }
        let modified = std::fs::metadata(&marker)
            .and_then(|metadata| metadata.modified())
            .unwrap_or(std::time::UNIX_EPOCH);
        entries.push(CachePruneEntry {
            path: path.to_path_buf(),
            bytes: directory_size(path),
            modified,
            kind: CachePruneEntryKind::Directory,
        });
    }
}

pub fn directory_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
