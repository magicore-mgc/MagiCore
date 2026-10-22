//! Package cache for downloaded tarballs and metadata.
//! Cache package cho tarball đã tải và metadata.

use super::entry::{TempPathGuard, create_unique_temp_file, publish_cache_entry, unique_temp_path};
use super::path::{metadata_path as metadata_cache_path, tarball_path as tarball_cache_path};
use anyhow::Result;
use mgc_types::PackageId;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

/// Package cache for downloaded tarballs and metadata.
#[derive(Clone)]
pub struct PackageCache {
    root: PathBuf,
}

impl PackageCache {
    pub fn new(root: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    /// Path to cached tarball for a given package version
    pub fn tarball_path(&self, id: &PackageId) -> PathBuf {
        tarball_cache_path(&self.root, id)
    }

    /// Path to cached metadata JSON for a package
    pub fn metadata_path(&self, name: &str) -> PathBuf {
        metadata_cache_path(&self.root, name)
    }

    /// Check if a package version is cached
    pub fn contains_tarball(&self, id: &PackageId) -> bool {
        self.tarball_path(id).exists()
    }

    /// Check if package metadata is cached
    pub fn contains_metadata(&self, name: &str) -> bool {
        self.metadata_path(name).exists()
    }

    /// Cache a tarball to disk
    pub fn cache_tarball(&self, id: &PackageId, data: &[u8]) -> Result<()> {
        let path = self.tarball_path(id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let (tmp, mut file) = create_unique_temp_file(&path)?;
        let guard = TempPathGuard::new(tmp);
        file.write_all(data)?;
        drop(file);
        publish_cache_entry(guard.path(), &path)
    }

    /// Cache a tarball by linking/copying an existing tarball path.
    pub fn cache_tarball_from_path(&self, id: &PackageId, source: &std::path::Path) -> Result<()> {
        let path = self.tarball_path(id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        for _ in 0..16 {
            let tmp = unique_temp_path(&path);
            match std::fs::hard_link(source, &tmp) {
                Ok(()) => {
                    let guard = TempPathGuard::new(tmp);
                    return publish_cache_entry(guard.path(), &path);
                }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(_) => {
                    let mut destination =
                        match OpenOptions::new().write(true).create_new(true).open(&tmp) {
                            Ok(file) => file,
                            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                            Err(err) => return Err(err.into()),
                        };
                    let guard = TempPathGuard::new(tmp);
                    let mut source_file = File::open(source)?;
                    std::io::copy(&mut source_file, &mut destination)?;
                    drop(destination);
                    return publish_cache_entry(guard.path(), &path);
                }
            }
        }
        anyhow::bail!(
            "could not allocate a unique cache staging file for {}",
            path.display()
        )
    }

    /// Cache metadata JSON
    pub fn cache_metadata(&self, name: &str, data: &[u8]) -> Result<()> {
        let path = self.metadata_path(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let (tmp, mut file) = create_unique_temp_file(&path)?;
        let mut guard = TempPathGuard::new(tmp);
        file.write_all(data)?;
        drop(file);
        // Metadata is mutable (registry refreshes), so publish atomically with
        // last-writer-wins semantics but never share a staging name.
        std::fs::rename(guard.path(), path)?;
        guard.disarm();
        Ok(())
    }

    /// Compatibility helper that materializes a cached tarball into memory.
    /// Helper tương thích, đọc toàn bộ tarball cache vào RAM.
    /// Production cache-hit paths should prefer `open_tarball`.
    /// Luồng cache hit production nên ưu tiên `open_tarball`.
    pub fn get_tarball(&self, id: &PackageId) -> Result<Option<Vec<u8>>> {
        let Some(mut file) = self.open_tarball(id)? else {
            return Ok(None);
        };
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(Some(bytes))
    }

    /// Open a cached tarball as a file without following its final symlink.
    /// Mở tarball cache dạng file và không đi theo symlink cuối.
    pub fn open_tarball(&self, id: &PackageId) -> Result<Option<File>> {
        let path = self.tarball_path(id);
        match mgc_platform::open_regular_file_no_follow(&path) {
            Ok(file) => Ok(Some(file)),
            Err(mgc_platform::OpenRegularFileError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                Ok(None)
            }
            Err(error) => Err(error.into()),
        }
    }

    /// Clear all cached packages
    pub fn clear(&self) -> Result<()> {
        if self.root.exists() {
            std::fs::remove_dir_all(&self.root)?;
            std::fs::create_dir_all(&self.root)?;
        }
        Ok(())
    }

    /// Total number of cached tarballs
    pub fn tarball_count(&self) -> usize {
        walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "tgz"
            })
            .count()
    }

    /// Total disk usage in bytes
    pub fn disk_usage(&self) -> u64 {
        let mut total = 0u64;
        let entries = walkdir::WalkDir::new(&self.root).into_iter();
        for entry in entries.flatten() {
            // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
            if entry.file_type().is_file()
                && let Ok(meta) = entry.metadata()
            {
                total += meta.len();
            }
        }
        total
    }
}
