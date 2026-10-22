//! Extract ZIP entries under a safe destination — giải nén entry vào đích an toàn.

use super::super::read_zip_entries;
use super::super::zip_path::sanitize_zip_path;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Extract every regular-file entry into `dest` (safe: sanitized paths,
/// no links, size caps). Mirrors `archive.rs`'s extraction contract.
/// Giải nén mọi entry file thường vào `dest` (an toàn: path đã sanitize,
/// không link, ghim kích thước). Phản chiếu hợp đồng giải nén của `archive.rs`.
pub fn extract_zip(bytes: &[u8], dest: &Path) -> MgResult<()> {
    let entries = read_zip_entries(bytes)?;
    let dest_root = super::super::super::archive::ensure_extract_root(dest)?;
    for entry in &entries {
        let rel = sanitize_zip_path(&entry.name)?;
        let target = super::super::super::archive::ensure_extract_file_target(&dest_root, &rel)?;
        if !target.starts_with(&dest_root) {
            return Err(MgError::Other(format!(
                "zip entry escapes destination: {}",
                target.display()
            )));
        }
        super::super::super::archive::write_extracted_file(&target, &entry.data)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/extract_zip.rs"]
mod tests;
