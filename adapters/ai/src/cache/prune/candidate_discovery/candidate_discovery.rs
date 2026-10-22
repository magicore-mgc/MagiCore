//! Find safe direct cache entries — tìm entry trực tiếp có thể đánh giá an toàn.

use crate::cache::filesystem::logical_size::logical_size;
use crate::cache::metadata::{CacheEntry, load_metadata, metadata_path};
use crate::cache::model_cache_path::model_id_for_cache_entry;
use crate::cache::prune::error::{PruneError, PruneOutcome};
use crate::registry::Registry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(in crate::cache::prune) struct CacheCandidate {
    pub(in crate::cache::prune) path: PathBuf,
    pub(in crate::cache::prune) cache_model_id: String,
    pub(in crate::cache::prune) metadata: Option<CacheEntry>,
    pub(in crate::cache::prune) metadata_sidecar: Option<PathBuf>,
    pub(in crate::cache::prune) content_bytes: Option<u64>,
}

impl CacheCandidate {
    pub(in crate::cache::prune) fn model_id(&self) -> String {
        self.metadata
            .as_ref()
            .map(|metadata| metadata.model_id.as_str())
            .filter(|model_id| !model_id.is_empty() && *model_id != "unknown")
            .map(str::to_owned)
            .unwrap_or_else(|| self.cache_model_id.clone())
    }
}

pub(in crate::cache::prune) fn discover_candidates(
    registry: &Registry,
    cache: &Path,
    measure_content_size: bool,
) -> PruneOutcome<Vec<CacheCandidate>> {
    let entries =
        fs::read_dir(cache).map_err(|source| PruneError::io("read cache directory", source))?;
    let mut paths = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| PruneError::io("read cache directory entry", source))?;
        let file_type = entry
            .file_type()
            .map_err(|source| PruneError::io("inspect cache directory entry", source))?;

        if !file_type.is_file() && !file_type.is_dir() {
            continue;
        }

        let path = entry.path();
        let Some(entry_name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(cache_model_id) =
            model_id_for_cache_entry(registry, &entry_name, file_type.is_dir())
        else {
            continue;
        };

        paths.push((path, cache_model_id));
    }

    let mut sidecar_owners = HashMap::<PathBuf, usize>::new();
    for (path, _) in &paths {
        *sidecar_owners.entry(metadata_path(path)).or_default() += 1;
    }

    let mut candidates = Vec::with_capacity(paths.len());
    for (path, cache_model_id) in paths {
        let sidecar_path = metadata_path(&path);
        let sidecar_is_unique = sidecar_owners.get(&sidecar_path) == Some(&1);
        let metadata_sidecar = sidecar_is_unique.then_some(sidecar_path.clone());
        let metadata = if sidecar_is_unique {
            load_regular_metadata(&path, &sidecar_path)
        } else {
            None
        };
        let content_bytes = measure_content_size
            .then(|| logical_size(&path))
            .transpose()
            .map_err(PruneError::from)?;

        candidates.push(CacheCandidate {
            path,
            cache_model_id,
            metadata,
            metadata_sidecar,
            content_bytes,
        });
    }

    candidates.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(candidates)
}

fn load_regular_metadata(entry: &Path, sidecar: &Path) -> Option<CacheEntry> {
    match fs::symlink_metadata(sidecar) {
        Ok(metadata) if metadata.file_type().is_file() => load_metadata(&entry.to_path_buf()).ok(),
        Ok(_) => None,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            load_metadata(&entry.to_path_buf()).ok()
        }
        Err(_) => None,
    }
}
