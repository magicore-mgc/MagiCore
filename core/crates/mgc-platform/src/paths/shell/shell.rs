//! Shared managed-bin PATH API and OS handoff.
//! API PATH dùng chung cho bin do MGC quản lý và giao việc cho OS.

use std::path::{Path, PathBuf};

/// Persistently add the managed bin directory to the user's shell configuration.
/// Ghi bền vững thư mục bin vào cấu hình shell của user.
pub fn ensure_global_bin_on_path() -> anyhow::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("cannot resolve user home directory"))?;
    let shell = std::env::var("SHELL").unwrap_or_default();
    ensure_bin_on_path(&home, &shell, &super::super::global_bin_dir())
}

/// Add an existing, real managed bin directory to the selected user's PATH.
/// Thêm thư mục bin đã tồn tại, không phải symlink, vào PATH user đã chọn.
pub fn ensure_bin_on_path(home: &Path, shell: &str, bin: &Path) -> anyhow::Result<PathBuf> {
    let metadata = std::fs::symlink_metadata(bin)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        anyhow::bail!(
            "managed executable path '{}' must be a real directory",
            bin.display()
        );
    }
    let bin = bin.canonicalize()?;
    #[cfg(windows)]
    super::windows::ensure_windows_user_path(&bin)?;
    #[cfg(not(windows))]
    super::unix::ensure_bin_on_path(home, shell, &bin)?;
    Ok(bin)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
