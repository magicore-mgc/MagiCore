//! List model cache entries — liệt kê model entry, không lộ file điều khiển HF.

use super::error::ListCachedModelsError;
use crate::cache::cache_dir;
use crate::cache::model_cache_path::model_id_for_cache_entry;
use crate::registry::Registry;
use mgc_types::{MgError, MgResult};
use std::fs;
use std::path::Path;

/// List model identifiers recorded at the registry cache root.
/// Liệt kê model ID được ghi tại cache root của registry.
pub fn list_cached_models(registry: &Registry) -> MgResult<Vec<String>> {
    list_models_at(registry, &cache_dir(registry)?)
}

pub(in crate::cache) fn list_models_at(registry: &Registry, cache: &Path) -> MgResult<Vec<String>> {
    let metadata = match fs::metadata(cache) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => {
            return Err(MgError::from(ListCachedModelsError::io(
                "inspect cache root",
                source,
            )));
        }
    };
    if !metadata.is_dir() {
        return Err(ListCachedModelsError::CacheRootNotDirectory.into());
    }

    let mut models = Vec::new();
    let entries = fs::read_dir(cache)
        .map_err(|source| MgError::from(ListCachedModelsError::io("read cache root", source)))?;
    for entry in entries {
        let entry = entry.map_err(|source| {
            MgError::from(ListCachedModelsError::io(
                "read cache directory entry",
                source,
            ))
        })?;
        let file_type = entry.file_type().map_err(|source| {
            MgError::from(ListCachedModelsError::io(
                "inspect cache directory entry",
                source,
            ))
        })?;
        if file_type.is_symlink() || (!file_type.is_file() && !file_type.is_dir()) {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if let Some(model_id) = model_id_for_cache_entry(registry, &name, file_type.is_dir()) {
            models.push(model_id);
        }
    }

    models.sort();
    Ok(models)
}

#[cfg(test)]
#[path = "test/list_cached_models.rs"]
mod tests;
