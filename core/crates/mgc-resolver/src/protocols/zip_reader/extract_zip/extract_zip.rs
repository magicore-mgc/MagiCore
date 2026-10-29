//! Extract ZIP entries under a safe destination — giải nén entry vào đích an toàn.

use super::super::read_zip_entries;
use super::super::zip_path::sanitize_zip_path;
use crate::protocols::archive::InstallHookPolicy;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Extract every regular-file entry into `dest` (safe: sanitized paths,
/// no links, size caps). Mirrors `archive.rs`'s extraction contract.
///
/// SEC-3: wheels and nupkgs are ZIPs, so this is where a Python `.pth` /
/// `sitecustomize.py` or a NuGet `install.ps1` would otherwise land on disk.
/// The gate refuses those entries before the first file is written unless the
/// caller passed a policy that names the package and the operator opted in.
/// (Giải nén mọi entry file thường vào `dest` (an toàn: path đã sanitize, không
/// link, ghim kích thước). Phản chiếu hợp đồng giải nén của `archive.rs`.
/// SEC-3: wheel/nupkg là ZIP nên đây là chỗ `.pth`/`sitecustomize.py` của
/// Python hoặc `install.ps1` của NuGet sẽ rơi xuống đĩa. Cổng chặn các entry đó
/// trước khi ghi file đầu tiên trừ khi caller truyền policy nêu package và
/// operator đã opt-in.)
pub fn extract_zip(bytes: &[u8], dest: &Path) -> MgResult<()> {
    extract_zip_with_policy(bytes, dest, &InstallHookPolicy::deny())
}

/// Extract a ZIP with an explicit install-hook policy.
/// Giải nén ZIP với policy hook cài đặt tường minh.
pub fn extract_zip_with_policy(
    bytes: &[u8],
    dest: &Path,
    hooks: &InstallHookPolicy,
) -> MgResult<()> {
    let entries = read_zip_entries(bytes)?;
    // Gate first, write later: a refused archive must leave no trace on disk.
    // (Cổng trước, ghi sau: archive bị từ chối không được để lại dấu vết trên đĩa.)
    for entry in &entries {
        hooks
            .check(&entry.name, entry.unix_mode)
            .map_err(|hook| MgError::Integrity(hook.to_string()))?;
    }
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
