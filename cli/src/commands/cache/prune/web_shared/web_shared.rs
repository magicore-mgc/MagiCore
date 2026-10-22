//! Measure and prune shared Web package roots by live project references.
//! Đo và prune package root Web dựa trên reference project còn sống.

use crate::commands::cache::{
    filesystem::{canonical_or_original, cleanup_empty_dirs, path_size, remove_path},
    prune::web_shared::{
        package_roots::web_shared_package_roots,
        references::{count_web_shared_live_project_refs, read_web_shared_pinned_package_roots},
    },
    types::WebSharedCacheStats,
};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::cache) fn count_web_shared_unpinned_package_roots(
    root: &Path,
) -> Result<usize> {
    let pinned = read_web_shared_pinned_package_roots(root);
    let count = web_shared_package_roots(root)
        .into_iter()
        .filter(|path| !pinned.contains(&canonical_or_original(path)))
        .count();
    Ok(count)
}

pub(in crate::commands::cache) fn prune_web_shared_unpinned_package_roots(
    root: &Path,
) -> Result<usize> {
    let pinned = read_web_shared_pinned_package_roots(root);
    let mut removed = 0usize;
    for package_root in web_shared_package_roots(root) {
        if pinned.contains(&canonical_or_original(&package_root)) {
            continue;
        }
        remove_path(&package_root)?;
        removed += 1;
    }
    cleanup_empty_dirs(&root.join("packages"));
    Ok(removed)
}

pub(in crate::commands::cache) fn web_shared_cache_stats(root: &Path) -> WebSharedCacheStats {
    let pinned = read_web_shared_pinned_package_roots(root);
    let mut stats = WebSharedCacheStats {
        project_refs: count_web_shared_live_project_refs(root),
        ..Default::default()
    };

    for package_root in web_shared_package_roots(root) {
        let size = path_size(&package_root);
        if pinned.contains(&canonical_or_original(&package_root)) {
            stats.pinned_package_roots += 1;
            stats.pinned_package_bytes = stats.pinned_package_bytes.saturating_add(size);
        } else {
            stats.unpinned_package_roots += 1;
            stats.unpinned_package_bytes = stats.unpinned_package_bytes.saturating_add(size);
        }
    }
    stats
}
