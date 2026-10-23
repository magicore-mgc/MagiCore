//! Lib cache clearing.
//! Xoá cache lib.

use mgc_types::{MgError, MgResult};

use super::super::dir::cache_dir;

/// Clear cache for specific language.
/// Xóa cache cho ngôn ngữ cụ thể.
pub fn clear_cache(language: &str) -> MgResult<()> {
    let dir = cache_dir(language)?;
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|e| MgError::Other(format!("failed to remove cache: {}", e)))?;
    }
    Ok(())
}
