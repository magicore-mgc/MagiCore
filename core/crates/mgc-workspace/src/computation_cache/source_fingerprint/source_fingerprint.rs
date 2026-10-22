// Package source fingerprint coordinator — điều phối fingerprint source package

use super::super::{source_entries::hash_source_entries, source_walk::walk_and_collect_hashes};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

/// Collect and hash package source files, excluding generated and cache directories.
/// Thu thập và hash source package, bỏ qua thư mục sinh tự động và cache.
pub fn compute_package_source_hash(package_root: &Path) -> Result<String> {
    // Refuse missing, linked, or non-directory roots so they cannot look like empty packages.
    // Từ chối root bị thiếu, là symlink hoặc không phải thư mục để không bị hiểu nhầm là package rỗng.
    let root_metadata = fs::symlink_metadata(package_root).with_context(|| {
        format!(
            "cannot inspect package source root '{}'",
            package_root.display()
        )
    })?;
    if root_metadata.file_type().is_symlink() {
        bail!(
            "package source root '{}' must not be a symlink",
            package_root.display()
        );
    }
    if !root_metadata.is_dir() {
        bail!(
            "package source root '{}' must be a directory",
            package_root.display()
        );
    }

    let mut file_hashes: BTreeMap<String, String> = BTreeMap::new();
    let mut visited_directories = HashSet::new();
    walk_and_collect_hashes(
        package_root,
        package_root,
        &mut file_hashes,
        &mut visited_directories,
    )?;

    Ok(hash_source_entries(&file_hashes))
}
