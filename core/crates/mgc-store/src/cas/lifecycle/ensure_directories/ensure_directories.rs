//! Create the required CAS content-addressed directory tree.
//! Tạo cây thư mục CAS content-addressed cần thiết.
use std::fs;
use std::path::Path;

use crate::cas::store::StoreError;

/// Ensure the CAS directory tree exists (creating it with tight modes via
/// the permissions pass below).
/// Đảm bảo cây thư mục CAS tồn tại (tạo với mode chặt qua bước permission).
pub fn ensure_cas_dirs(root: &Path) -> Result<(), StoreError> {
    let dirs = [
        root.join("files").join("blake3"),
        root.join("compiled").join("blake3"),
    ];
    for dir in &dirs {
        fs::create_dir_all(dir).map_err(|e| StoreError::Io {
            path: dir.clone(),
            msg: e.to_string(),
        })?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
