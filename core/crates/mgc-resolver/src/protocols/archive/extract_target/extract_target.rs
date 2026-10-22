//! Validate contained archive file and directory targets.
//! Xác minh file và thư mục đích nằm an toàn trong archive root.

use super::super::{ArchiveError, ArchiveResult};
use std::path::{Path, PathBuf};

/// Create each parent directory one component at a time and reject any
/// symlink/reparse-like component already present below the extraction root.
/// This prevents a valid archive path such as `nested/file` from following a
/// pre-planted `nested -> outside` link. This is defense against pre-existing
/// links; directory-handle/openat hardening for a concurrent same-user path
/// swap remains a separate platform-specific layer.
/// Tạo từng parent và từ chối thành phần symlink/reparse-like có sẵn dưới root.
/// Điều này ngăn path hợp lệ như `nested/file` đi theo link `nested -> outside`.
/// Biện pháp này chặn link có sẵn; chưa xử lý swap path đồng thời bởi cùng user.
/// Hardening bằng directory handle/openat vẫn là lớp riêng theo platform.
pub(in crate::protocols::archive) fn ensure_extract_file_target(
    root: &Path,
    relative: &Path,
) -> ArchiveResult<PathBuf> {
    let (parent, filename) = super::super::archive_path::split_safe_relative(relative)?;
    let mut current = root.to_path_buf();
    for component in parent {
        current.push(component);
        super::super::archive_path::ensure_directory_component(&current)?;
    }
    current.push(filename);
    match std::fs::symlink_metadata(&current) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(ArchiveError::Integrity(format!(
                "refusing archive file target symlink: {}",
                current.display()
            )));
        }
        Ok(metadata) if !metadata.is_file() => {
            return Err(ArchiveError::Integrity(format!(
                "refusing non-file archive target: {}",
                current.display()
            )));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    Ok(current)
}

/// Ensure an archive directory is a real directory, never a symlink.
/// Đảm bảo thư mục archive là thư mục thật, không phải symlink.
pub(in crate::protocols::archive) fn ensure_extract_directory(
    root: &Path,
    relative: &Path,
) -> ArchiveResult<PathBuf> {
    let (parent, filename) = super::super::archive_path::split_safe_relative(relative)?;
    let mut current = root.to_path_buf();
    for component in parent {
        current.push(component);
        super::super::archive_path::ensure_directory_component(&current)?;
    }
    current.push(filename);
    super::super::archive_path::ensure_directory_component(&current)?;
    Ok(current)
}
