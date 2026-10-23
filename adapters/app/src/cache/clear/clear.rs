//! Cache clearing for one App language.
//! Xoá cache cho một ngôn ngữ App.

use crate::language::AppLanguage;
use mgc_types::{MgError, MgResult};

use super::super::path::cache_dir;

/// Clear cache for specific language.
pub fn clear_cache(language: AppLanguage) -> MgResult<()> {
    let dir = cache_dir(language)?;
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|e| MgError::Other(format!("failed to remove cache: {}", e)))?;
    }
    Ok(())
}
