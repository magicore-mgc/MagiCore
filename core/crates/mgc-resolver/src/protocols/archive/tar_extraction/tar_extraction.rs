//! General safe tar.gz extraction.
//! Giải nén tar.gz tổng quát theo quy tắc an toàn.

use super::super::archive::{
    MAX_TAR_ENTRIES, MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED,
};
use super::super::install_hook::InstallHookPolicy;
use super::super::{ArchiveError, ArchiveResult};
use std::collections::VecDeque;
use std::io::Read as _;
use std::path::Path;

/// Extract a gzip-compressed tarball (held in memory) into `dest`.
///
/// The caller supplies the install-hook policy so this path and the fetcher
/// path can never disagree on whether `.pth`/`sitecustomize.py`/`install.ps1`
/// entries are acceptable.
/// (Giải nén tarball gzip trong bộ nhớ vào `dest`. Caller truyền policy hook
/// cài đặt để đường này và đường fetcher không bao giờ lệch nhau về việc chấp
/// nhận entry `.pth`/`sitecustomize.py`/`install.ps1`.)
#[allow(private_interfaces)] // ArchiveError stays crate-private; the facade returns MgError
pub(crate) fn extract_tar_gz(
    bytes: &[u8],
    dest: &Path,
    hooks: &InstallHookPolicy,
) -> ArchiveResult<()> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);

    let dest_root = super::super::extract_root::ensure_extract_root(dest)?;
    let mut total_uncompressed = 0_u64;
    let mut entry_count = 0_usize;
    // Staging queue preserves tar's last-entry-wins order while letting every
    // entry be validated before the first file is written.
    // (Hàng đợi staging giữ thứ tự last-entry-wins của tar, đồng thời cho phép
    // xác minh mọi entry trước khi ghi file đầu tiên.)
    let mut pending: VecDeque<(std::path::PathBuf, Vec<u8>)> = VecDeque::new();

    for entry in archive.entries()? {
        let mut entry = entry?;
        entry_count = entry_count
            .checked_add(1)
            .ok_or_else(|| ArchiveError::Other("tar entry count overflow".to_string()))?;
        if entry_count > MAX_TAR_ENTRIES {
            return Err(ArchiveError::Other(format!(
                "tar archive exceeds the {MAX_TAR_ENTRIES} entry limit"
            )));
        }
        let rel_path = super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let entry_type = entry.header().entry_type();
        // SEC-3: refuse interpreter install hooks and privileged modes before
        // any byte reaches the destination. Exempt pax metadata because those
        // names are bookkeeping, not package files.
        // (SEC-3: từ chối hook cài đặt interpreter và mode đặc quyền trước khi
        // byte nào tới đích. Bỏ qua metadata pax vì đó là bookkeeping.)
        if !matches!(entry_type.as_byte(), b'g' | b'x') {
            hooks.check(&rel_path.to_string_lossy(), entry.header().mode().ok())?;
        }
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ArchiveError::Other(format!(
                "tar links are not allowed: {}",
                rel_path.display()
            )));
        }
        let target = if entry_type.is_dir() {
            super::super::extract_target::ensure_extract_directory(&dest_root, &rel_path)?
        } else {
            super::super::extract_target::ensure_extract_file_target(&dest_root, &rel_path)?
        };

        if !target.starts_with(&dest_root) {
            return Err(ArchiveError::Other(format!(
                "tar entry escapes destination: {}",
                target.display()
            )));
        }

        if entry_type.is_dir() {
            continue;
        }

        // pax metadata entries are archive bookkeeping, not package files.
        // (Entry metadata pax là bookkeeping archive, không phải file package.)
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }

        if !entry_type.is_file() {
            return Err(ArchiveError::Other(format!(
                "unsupported tar entry type for {}",
                target.display()
            )));
        }

        let size = entry.size();
        if size > MAX_TAR_ENTRY_UNCOMPRESSED {
            return Err(ArchiveError::Other(format!(
                "tar entry '{}' exceeds the per-entry size limit",
                rel_path.display()
            )));
        }
        total_uncompressed = total_uncompressed.checked_add(size).ok_or_else(|| {
            ArchiveError::Other("tar total uncompressed size overflow".to_string())
        })?;
        if total_uncompressed > MAX_TAR_TOTAL_UNCOMPRESSED {
            return Err(ArchiveError::Other(
                "tar total uncompressed size exceeds the safety limit".to_string(),
            ));
        }

        let mut contents = Vec::with_capacity(size as usize);
        entry.read_to_end(&mut contents)?;
        // Bounded by MAX_TAR_TOTAL_UNCOMPRESSED, so buffering the archive's
        // payload in memory is capped; nothing is written until every entry
        // has passed path, link, size and hook checks.
        // (Bị chặn bởi MAX_TAR_TOTAL_UNCOMPRESSED nên buffer trong RAM bị giới
        // hạn; chưa ghi gì cho tới khi mọi entry qua hết kiểm tra.)
        pending.push_back((target, contents));
    }

    for (target, contents) in pending {
        super::super::extracted_file_write::write_extracted_file(&target, &contents)?;
    }

    Ok(())
}
