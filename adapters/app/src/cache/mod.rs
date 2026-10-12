//! `cache/mod.rs` — Cache management for app adapter.
//! MGC-owned app cache paths; unsupported lanes cannot claim external caches.

pub mod metadata;
pub mod prune;

use crate::language::AppLanguage;
use mgc_types::{MgError, MgResult};
use std::path::PathBuf;

/// Get app adapter cache directory for language.
pub fn cache_dir(language: AppLanguage) -> MgResult<PathBuf> {
    match language {
        AppLanguage::Flutter => Ok(mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("pub")),
        AppLanguage::Swift => Ok(mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("swift")),
        AppLanguage::Multi => Ok(mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("app")
            .join("multi")),
        // These ecosystems use shared external package-manager caches. Until
        // MagiCore owns those stores, refusing is safer than offering a clear
        // operation that could delete data used by unrelated projects.
        AppLanguage::Kotlin
        | AppLanguage::ReactNative
        | AppLanguage::ObjC
        | AppLanguage::Maui
        | AppLanguage::Tauri => Err(MgError::Unsupported {
            core: "app",
            capability: "managed-cache",
            guidance: format!(
                "MagiCore does not manage the external {} package cache",
                language.as_str()
            ),
        }),
    }
}

/// Clear cache for specific language.
pub fn clear_cache(language: AppLanguage) -> MgResult<()> {
    let dir = cache_dir(language)?;
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|e| MgError::Other(format!("failed to remove cache: {}", e)))?;
    }
    Ok(())
}

/// Get cache size for specific language.
pub fn cache_size(language: AppLanguage) -> MgResult<u64> {
    let dir = cache_dir(language)?;
    if !dir.exists() {
        return Ok(0);
    }
    dir_size(&dir)
}

fn dir_size(path: &std::path::Path) -> MgResult<u64> {
    // Do not traverse linked roots or entries — không đi ra ngoài cây cache qua symlink.
    let root = std::fs::symlink_metadata(path)
        .map_err(|e| MgError::Other(format!("failed to inspect cache root: {e}")))?;
    if root.file_type().is_symlink() || !root.is_dir() {
        return Err(MgError::Other("cache root must be a real directory".into()));
    }
    let mut total = 0u64;
    for entry in
        std::fs::read_dir(path).map_err(|e| MgError::Other(format!("failed to read dir: {}", e)))?
    {
        let entry = entry.map_err(|e| MgError::Other(format!("failed to read entry: {}", e)))?;
        let metadata = std::fs::symlink_metadata(entry.path())
            .map_err(|e| MgError::Other(format!("failed to read metadata: {}", e)))?;

        if metadata.file_type().is_symlink() {
            continue;
        }
        let size = if metadata.is_dir() {
            dir_size(&entry.path())?
        } else if metadata.is_file() {
            metadata.len()
        } else {
            0
        };
        total = total
            .checked_add(size)
            .ok_or_else(|| MgError::Other("cache size exceeds supported range".into()))?;
    }
    Ok(total)
}

#[cfg(test)]
#[path = "test/cache_size.rs"]
mod cache_size_tests;
