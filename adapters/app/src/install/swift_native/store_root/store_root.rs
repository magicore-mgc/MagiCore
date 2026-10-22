//! Select and create the MGC-managed Swift checkout root.
//! Chọn và tạo gốc checkout Swift do MGC quản lý.

use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// Resolve the environment override lazily, otherwise use the shared MGC data root.
/// Resolve override môi trường theo kiểu lazy, nếu thiếu dùng gốc data MGC chung.
pub(in crate::install) fn swift_store_root() -> MgResult<PathBuf> {
    let override_root = std::env::var("MGC_SWIFT_STORE_ROOT").ok();
    let root = override_store_root(override_root)
        .unwrap_or_else(|| default_store_root(&mgc_platform::paths::mgc_data_dir()));
    std::fs::create_dir_all(&root)
        .map_err(|error| MgError::Other(format!("create swift store root: {error}")))?;
    Ok(root)
}

/// Preserve whitespace-only override rejection and the original non-empty spelling.
/// Giữ việc từ chối override toàn khoảng trắng và spelling nguyên gốc khi không rỗng.
fn override_store_root(raw: Option<String>) -> Option<PathBuf> {
    raw.filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}

/// Construct the shared Swift store location without consulting process environment.
/// Tạo path Swift store dùng chung mà không đọc environment của process.
fn default_store_root(data_root: &Path) -> PathBuf {
    data_root.join("store").join("swift")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
