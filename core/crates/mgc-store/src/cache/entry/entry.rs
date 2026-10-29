//! Creates private staging paths and publishes cache entries atomically.
//! Tạo staging path riêng và publish entry cache nguyên tử.

use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

pub(in crate::cache) struct TempPathGuard {
    path: PathBuf,
    armed: bool,
}

impl TempPathGuard {
    pub(in crate::cache) fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    pub(in crate::cache) fn path(&self) -> &Path {
        &self.path
    }

    pub(in crate::cache) fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TempPathGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub(in crate::cache) fn unique_temp_path(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("cache-entry");
    parent.join(format!(
        ".{name}.tmp-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}

pub(in crate::cache) fn create_unique_temp_file(path: &Path) -> Result<(PathBuf, File)> {
    for _ in 0..16 {
        let temp = unique_temp_path(path);
        match OpenOptions::new().write(true).create_new(true).open(&temp) {
            Ok(file) => return Ok((temp, file)),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err.into()),
        }
    }
    anyhow::bail!(
        "could not allocate a unique cache staging file for {}",
        path.display()
    )
}

fn same_file_contents(left: &Path, right: &Path) -> Result<bool> {
    let left_meta = std::fs::symlink_metadata(left)?;
    let right_meta = std::fs::symlink_metadata(right)?;
    if !left_meta.file_type().is_file() || !right_meta.file_type().is_file() {
        return Ok(false);
    }
    if left_meta.len() != right_meta.len() {
        return Ok(false);
    }

    let mut left = File::open(left)?;
    let mut right = File::open(right)?;
    let mut left_hash = blake3::Hasher::new();
    let mut right_hash = blake3::Hasher::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = left.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        left_hash.update(&buffer[..read]);
    }
    loop {
        let read = right.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        right_hash.update(&buffer[..read]);
    }
    Ok(left_hash.finalize() == right_hash.finalize())
}

pub(in crate::cache) fn publish_cache_entry(temp: &Path, path: &Path) -> Result<()> {
    match std::fs::hard_link(temp, path) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            if !same_file_contents(temp, path)? {
                anyhow::bail!(
                    "cache conflict at {}: concurrent writers produced different content",
                    path.display()
                );
            }
        }
        Err(err) => return Err(err.into()),
    }
    std::fs::remove_file(temp)?;
    Ok(())
}

/// Publish a cache entry at a path whose parent is a mutable mirror slot.
/// Publish entry cache vào path có parent là slot mirror có thể thay.
///
/// A scope directory that disagrees with the publisher is a previous entry
/// from the same registry (for example after an upstream re-publish). Replace
/// it atomically instead of failing the whole download: the caller has already
/// verified the new bytes against integrity evidence.
/// Thư mục scope khác với publisher là entry cũ của cùng registry (ví dụ sau
/// khi upstream publish lại). Thay nguyên tử thay vì làm hỏng cả lần tải:
/// caller đã verify bytes mới theo bằng chứng integrity.
pub(in crate::cache) fn replace_cache_entry(temp: &Path, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    match std::fs::rename(temp, path) {
        Ok(()) => Ok(()),
        Err(err) => {
            // Windows refuses to replace an open file; fall back to remove+rename.
            // Windows từ chối thay file đang mở; fallback xoá rồi đổi tên.
            if path.exists() {
                std::fs::remove_file(path)?;
                std::fs::rename(temp, path)?;
                return Ok(());
            }
            Err(err.into())
        }
    }
}
