//! Package cache for downloaded tarballs and metadata.
//! Cache package cho tarball đã tải và metadata.

use super::entry::{
    TempPathGuard, create_unique_temp_file, publish_cache_entry, replace_cache_entry,
    unique_temp_path,
};
use super::path::{
    metadata_path as metadata_cache_path, registry_scope_directory,
    tarball_path as tarball_cache_path,
};
use anyhow::Result;
use mgc_types::PackageId;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

/// Package cache for downloaded tarballs and metadata.
///
/// The on-disk root may be narrowed to a registry scope: two registries that
/// publish different bytes for the same `name@version` must not share cache
/// entries (SEC-01). The scope is an opaque digest directory, so the root
/// path itself stays free of URLs and credentials.
/// Cache package cho tarball đã tải và metadata.
/// Root trên đĩa có thể bị thu hẹp theo scope registry: hai registry publish
/// bytes khác nhau cho cùng `name@version` không được dùng chung entry.
#[derive(Clone)]
pub struct PackageCache {
    root: PathBuf,
    base_root: PathBuf,
}

impl PackageCache {
    pub fn new(root: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root: root.clone(),
            base_root: root,
        })
    }

    /// Open a cache narrowed to one registry origin (SEC-01).
    /// Mở cache giới hạn theo một origin registry (SEC-01).
    ///
    /// Releases are identified by (registry, name, version): an entry written
    /// from a private mirror is never served to a project that resolves the
    /// same id from the public registry.
    /// Bản phát hành được định danh bằng (registry, name, version): entry ghi
    /// từ mirror riêng không bao giờ được phục vụ cho project resolve cùng id
    /// từ registry công khai.
    pub fn for_registry(root: PathBuf, registry: &str) -> Result<Self> {
        let scoped = registry_scope_directory(&root, registry);
        std::fs::create_dir_all(&scoped)?;
        Ok(Self {
            root: scoped,
            base_root: root,
        })
    }

    /// Narrow an existing cache to a registry origin, keeping its base root.
    /// Thu hẹp cache hiện có theo một origin registry, giữ base root.
    pub fn with_registry_scope(&self, registry: &str) -> Result<Self> {
        let scoped = registry_scope_directory(&self.base_root, registry);
        std::fs::create_dir_all(&scoped)?;
        Ok(Self {
            root: scoped,
            base_root: self.base_root.clone(),
        })
    }

    /// Root directory this cache instance reads and writes.
    /// Thư mục root mà instance cache này đọc và ghi.
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Unscoped root that all registry scopes of this cache live under.
    /// Root chưa scope mà mọi scope registry của cache này nằm dưới.
    pub fn base_root(&self) -> &PathBuf {
        &self.base_root
    }

    /// Path to cached tarball for a given package version
    pub fn tarball_path(&self, id: &PackageId) -> PathBuf {
        tarball_cache_path(&self.root, id)
    }

    /// Path to cached metadata JSON for a package.
    /// Đường dẫn metadata JSON của package.
    ///
    /// Metadata lives beside the tarballs in the same scope: a shared cache
    /// root may be pointed at by `MGC_CACHE_DIR` from tests and CI, and a
    /// foreign packument must not be served to another project.
    /// (Metadata nằm cạnh tarball trong cùng scope: root cache dùng chung có
    /// thể bị `MGC_CACHE_DIR` từ test/CI trỏ vào, và packument lạ không được
    /// phục vụ cho project khác.)
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

    /// Replace a tarball whose publisher is certified by integrity evidence.
    /// Thay tarball đã được integrity chứng thực nguồn.
    ///
    /// Used when the scope already holds bytes from the same registry that no
    /// longer match (upstream re-publish, partial write): the new bytes are
    /// written atomically over the stale entry instead of aborting the install.
    /// Dùng khi scope đã có bytes của cùng registry không còn khớp (upstream
    /// publish lại, ghi dở): bytes mới được ghi nguyên tử đè entry cũ thay vì
    /// huỷ lần cài.
    pub fn replace_verified_tarball(&self, id: &PackageId, data: &[u8]) -> Result<()> {
        let path = self.tarball_path(id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let (tmp, mut file) = create_unique_temp_file(&path)?;
        let guard = TempPathGuard::new(tmp);
        file.write_all(data)?;
        drop(file);
        replace_cache_entry(guard.path(), &path)?;
        Ok(())
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
