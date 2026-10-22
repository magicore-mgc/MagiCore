//! Measure and prune the Web project cache breakdown.
//! Đo và prune CAS, tarball, resolution của cache project Web.

use crate::commands::cache::{
    filesystem::path_size,
    prune::{prune_cas_blobs_under, prune_files_under},
    types::{WebProjectCacheStats, WebProjectPruneStats},
};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::cache) fn prune_web_project_cache(
    root: &Path,
    dry_run: bool,
) -> Result<WebProjectPruneStats> {
    Ok(WebProjectPruneStats {
        cas_files: prune_cas_blobs_under(&root.join("cas"), root, dry_run)?,
        tarball_files: prune_files_under(&root.join("cache"), dry_run)?,
        resolution_files: prune_files_under(&root.join("resolutions"), dry_run)?,
    })
}

pub(in crate::commands::cache) fn web_project_cache_stats(root: &Path) -> WebProjectCacheStats {
    WebProjectCacheStats {
        cas_bytes: path_size(&root.join("cas")),
        tarball_bytes: path_size(&root.join("cache")),
        resolution_bytes: path_size(&root.join("resolutions")),
    }
}
