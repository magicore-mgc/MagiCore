//! Prune cache files through explicit file predicates.
//! Prune file cache theo điều kiện và giữ hardlink ngoài.

use crate::commands::cache::filesystem::remove_path;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub(in crate::commands::cache) fn prune_files_under(root: &Path, dry_run: bool) -> Result<usize> {
    prune_files_under_with(root, dry_run, |_| true)
}

pub(in crate::commands::cache) fn prune_unlinked_files_under(
    root: &Path,
    dry_run: bool,
) -> Result<usize> {
    prune_files_under_with(root, dry_run, file_has_no_external_hardlinks)
}

fn prune_files_under_with<F>(root: &Path, dry_run: bool, should_prune: F) -> Result<usize>
where
    F: Fn(&Path) -> bool,
{
    if !root.exists() {
        return Ok(0);
    }
    let mut pruned = 0usize;
    let mut directories = Vec::new();
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            directories.push(entry.path().to_path_buf());
            continue;
        }
        if !entry.file_type().is_file() || !should_prune(entry.path()) {
            continue;
        }
        pruned += 1;
        if !dry_run {
            remove_path(entry.path())?;
        }
    }
    if !dry_run {
        directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
        for dir in directories {
            let _ = std::fs::remove_dir(&dir);
        }
    }
    Ok(pruned)
}

pub(in crate::commands::cache) use mgc_store::index::file_has_no_external_hardlinks;
