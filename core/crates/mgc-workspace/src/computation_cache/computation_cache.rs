// Freshness-check coordinator — điều phối kiểm tra freshness

use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;

use super::{
    cache_file::load_package_build_cache, dependency_fingerprint::compute_composite_hash,
    source_fingerprint::compute_package_source_hash,
};

/// Check whether a package needs to be rebuilt.
/// Kiểm tra xem package có cần build lại hay không.
/// Return `(should_rebuild, current_source_hash, current_composite_hash)`.
/// Trả về `(should_rebuild, current_source_hash, current_composite_hash)`.
pub fn check_package_build_freshness(
    package_root: &Path,
    dependency_hashes: &BTreeMap<String, String>,
) -> Result<(bool, String, String)> {
    let current_source_hash = compute_package_source_hash(package_root)?;
    let current_composite_hash = compute_composite_hash(&current_source_hash, dependency_hashes);

    let Some(saved_cache) = load_package_build_cache(package_root) else {
        return Ok((true, current_source_hash, current_composite_hash));
    };

    if saved_cache.composite_hash == current_composite_hash {
        // The cache is fresh when neither source nor dependencies changed.
        // Cache còn mới nếu source và dependencies đều không thay đổi.
        Ok((false, current_source_hash, current_composite_hash))
    } else {
        Ok((true, current_source_hash, current_composite_hash))
    }
}
