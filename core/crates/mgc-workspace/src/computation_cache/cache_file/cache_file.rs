// Package-local cache-file IO — I/O file cache cục bộ package

use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::super::{dependency_fingerprint::compute_composite_hash, record::PackageBuildCache};

const CACHE_FILE_NAME: &str = ".mgc_build_cache.json";

/// Read the saved computation cache for a package.
/// Đọc cache computation đã lưu của package.
pub fn load_package_build_cache(package_root: &Path) -> Option<PackageBuildCache> {
    let cache_path = package_root.join(".magicore").join(CACHE_FILE_NAME);
    let content = fs::read_to_string(cache_path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save a package computation cache after a successful build.
/// Lưu cache computation của package sau khi build thành công.
pub fn save_package_build_cache(
    package_root: &Path,
    source_hash: String,
    dependency_hashes: BTreeMap<String, String>,
) -> Result<PackageBuildCache> {
    let composite_hash = compute_composite_hash(&source_hash, &dependency_hashes);
    let cache_dir = package_root.join(".magicore");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }

    let cache = PackageBuildCache {
        source_hash,
        dependency_hashes,
        composite_hash,
        last_built_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
    };

    let content = serde_json::to_string_pretty(&cache)?;
    fs::write(cache_dir.join(CACHE_FILE_NAME), content)?;
    Ok(cache)
}
