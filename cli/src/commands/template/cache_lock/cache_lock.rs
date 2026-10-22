use super::error;
use crate::commands::template::template_cache_is_link;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Serialize template cache updates across MagiCore processes.
/// Tuần tự hóa cập nhật cache template giữa các process MagiCore.
pub(crate) fn lock_template_cache_layer(cache_root: &Path, rel: &str) -> Result<fs::File> {
    let lock_dir = cache_root.join(".locks");
    fs::create_dir_all(&lock_dir)?;
    let lock_dir_metadata = fs::symlink_metadata(&lock_dir)?;
    if template_cache_is_link(&lock_dir_metadata) || !lock_dir_metadata.is_dir() {
        return Err(error::lock_directory_is_not_real());
    }
    let key = blake3::hash(rel.as_bytes()).to_hex().to_string();
    // Keep the historical prefix so older and newer MGC processes share the same lock.
    // Giữ prefix cũ để process MGC phiên bản cũ và mới khóa chung một file.
    let lock_path = lock_dir.join(format!("embedded-{key}.lock"));
    let mut options = fs::OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // FILE_FLAG_OPEN_REPARSE_POINT: open the lock object, not its target.
        // (Mở chính lock, không đi theo reparse point trên Windows.)
        options.custom_flags(0x0020_0000);
    }
    let file = options.open(&lock_path)?;
    let metadata = fs::symlink_metadata(&lock_path)?;
    if !metadata.is_file() || template_cache_is_link(&metadata) || !file.metadata()?.is_file() {
        return Err(error::lock_file_is_not_regular());
    }
    fs2::FileExt::lock_exclusive(&file)?;
    Ok(file)
}
