//! Cache size accounting that never traverses links.
//! Đếm dung lượng cache, không đi qua liên kết.

use crate::language::AppLanguage;
use mgc_types::{MgError, MgResult};

use super::super::path::cache_dir;

/// Get cache size for specific language.
pub fn cache_size(language: AppLanguage) -> MgResult<u64> {
    let dir = cache_dir(language)?;
    if !dir.exists() {
        return Ok(0);
    }
    dir_size(&dir)
}

pub(crate) fn dir_size(path: &std::path::Path) -> MgResult<u64> {
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
