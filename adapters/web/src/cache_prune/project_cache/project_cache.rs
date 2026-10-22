//! Prune project-local package cache and unlinked CAS blobs.
//! Dọn package cache theo project và blob CAS không còn hard-link.

use std::path::Path;

use mgc_store::Layout;
use walkdir::WalkDir;

use crate::cache_prune::{
    path_is_older_than, project_cache_prune_due, prune_old_files_under, remove_dir_if_empty,
    shared_cache_max_age_secs, write_project_cache_prune_stamp,
};
use mgc_types::MgResult;

pub fn prune_project_local_cache(layout: &Layout) {
    if !project_cache_prune_due(layout.root()) {
        return;
    }
    let max_age = std::time::Duration::from_secs(shared_cache_max_age_secs());
    let _ = prune_old_files_under(&layout.cache_dir(), max_age);
    let _ = prune_unlinked_old_cas_files_under(&layout.cas_dir(), max_age);
    let _ = prune_old_files_under(&layout.root().join("resolutions"), max_age);
    let _ = write_project_cache_prune_stamp(layout.root());
}

pub fn prune_unlinked_old_cas_files_under(
    root: &Path,
    max_age: std::time::Duration,
) -> MgResult<()> {
    if !root.exists() {
        return Ok(());
    }
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
        if !entry.file_type().is_file()
            || !path_is_older_than(entry.path(), max_age)
            || !file_has_no_external_hardlinks(entry.path())
        {
            continue;
        }
        let _ = std::fs::remove_file(entry.path());
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for dir in directories {
        remove_dir_if_empty(&dir);
    }
    Ok(())
}

pub fn file_has_no_external_hardlinks(path: &Path) -> bool {
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
