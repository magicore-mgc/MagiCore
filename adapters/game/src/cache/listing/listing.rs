//! Cached engine version listing.
//! Liệt kê version engine đã cache.

use crate::engine::GameEngine;
use mgc_types::MgResult;

use super::super::dir::cache_dir;

/// List cached versions for engine
pub fn list_cached_versions(engine: GameEngine) -> MgResult<Vec<String>> {
    let cache = cache_dir(engine)?;

    if !cache.exists() {
        return Ok(vec![]);
    }

    let mut versions = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&cache) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                // Extract version from "godot-4.3.0" → "4.3.0"
                if let Some(version) = name.split('-').nth(1) {
                    versions.push(version.to_string());
                }
            }
        }
    }

    Ok(versions)
}
