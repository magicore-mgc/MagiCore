//! Coordinate AI cache pruning — điều phối discover, chọn policy và xóa entry.

use super::candidate_discovery::discover_candidates;
use super::error::PruneError;
use super::policy::select_removals;
use super::removal::{remove_candidate, remove_model_path};
use super::types::{PruneResult, PruneStrategy};
use crate::cache::{cache_dir, model_cache_path};
use crate::registry::Registry;
use mgc_types::{MgError, MgResult};
use std::fs;
use std::path::PathBuf;

/// Remove cache entries selected by the requested strategy.
/// Xóa các cache entry được strategy chỉ định lựa chọn.
pub fn prune_cache(registry: &Registry, strategy: PruneStrategy) -> MgResult<PruneResult> {
    let cache = cache_dir(registry)?;
    match fs::metadata(&cache) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => return Err(PruneError::CacheRootNotDirectory.into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(empty_result());
        }
        Err(source) => {
            return Err(PruneError::io("inspect cache directory", source).into());
        }
    }

    let measure_content_size = matches!(strategy, PruneStrategy::LargerThan(_));
    let candidates =
        discover_candidates(registry, &cache, measure_content_size).map_err(MgError::from)?;
    let mut removal_indexes = select_removals(&candidates, strategy);
    removal_indexes.sort_by(|left, right| candidates[*left].path.cmp(&candidates[*right].path));

    let mut result = empty_result();
    for index in removal_indexes {
        let candidate = &candidates[index];
        if let Some(bytes) = remove_candidate(candidate).map_err(MgError::from)? {
            result.removed_count += 1;
            result.bytes_freed = result.bytes_freed.saturating_add(bytes);
            result.removed_models.push(candidate.model_id());
        }
    }

    Ok(result)
}

/// Remove one model by its registry identifier and return its logical bytes.
/// Xóa một model theo ID registry và trả logical bytes của file đã xóa.
pub fn remove_model(registry: &Registry, model_id: &str) -> MgResult<u64> {
    let path: PathBuf = model_cache_path(registry, model_id)?;
    Ok(remove_model_path(&path)
        .map_err(MgError::from)?
        .unwrap_or_default())
}

fn empty_result() -> PruneResult {
    PruneResult {
        removed_count: 0,
        bytes_freed: 0,
        removed_models: Vec::new(),
    }
}
