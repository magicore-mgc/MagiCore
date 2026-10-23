//! Lib cache size accounting.
//! Đếm dung lượng cache lib.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::dir::cache_dir;

/// Get cache size for specific language.
/// Lấy kích thước cache cho ngôn ngữ cụ thể.
pub fn cache_size(language: &str) -> MgResult<u64> {
    let dir = cache_dir(language)?;
    if !dir.exists() {
        return Ok(0);
    }
    dir_size(&dir)
}

/// Recursively calculate directory size.
/// Tính kích thước thư mục đệ quy.
fn dir_size(path: &Path) -> MgResult<u64> {
    let mut total = 0u64;
    for entry in
        std::fs::read_dir(path).map_err(|e| MgError::Other(format!("failed to read dir: {}", e)))?
    {
        let entry = entry.map_err(|e| MgError::Other(format!("failed to read entry: {}", e)))?;
        let metadata = entry
            .metadata()
            .map_err(|e| MgError::Other(format!("failed to read metadata: {}", e)))?;

        if metadata.is_dir() {
            total += dir_size(&entry.path())?;
        } else {
            total += metadata.len();
        }
    }
    Ok(total)
}
