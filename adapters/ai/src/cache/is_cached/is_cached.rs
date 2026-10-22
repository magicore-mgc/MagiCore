//! Check whether a model path resolves in its registry cache — kiểm tra model có resolve trong cache.

use super::error::IsCachedError;
use crate::cache::model_cache_path;
use crate::registry::Registry;
use mgc_types::{MgError, MgResult};
use std::fs;

/// Return true when the model cache path resolves to an existing filesystem entry.
/// Trả true khi model cache path resolve tới filesystem entry đang tồn tại.
pub fn is_cached(registry: &Registry, model_id: &str) -> MgResult<bool> {
    let path = model_cache_path(registry, model_id)?;
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(source) => Err(MgError::from(IsCachedError::io(source))),
    }
}
