//! Package-cache state, constructors and stable path derivation.
//! Trạng thái cache package, constructor và cách tạo path ổn định.

use crate::package_cache::lock_shards::PackageLocks;
use crate::package_cache::path_guard::ensure_cache_directory;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Maximum legacy component length accepted for backward-compatible cache reads.
/// Giới hạn độ dài component cũ được chấp nhận khi đọc tương thích.
const MAX_LEGACY_COMPONENT_BYTES: usize = 180;

/// Shared package cache — Cache package dùng chung.
pub struct PackageCache {
    pub(in crate::package_cache) root: PathBuf,
    pub(in crate::package_cache) locks: PackageLocks,
}

impl PackageCache {
    /// Create a cache at the managed default location — Tạo cache ở vị trí mặc định được quản lý.
    pub fn new() -> Result<Self> {
        let data_root = mgc_platform::paths::mgc_data_dir();
        let root = Self::default_cache_dir()?;
        ensure_cache_directory(&data_root, &root)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::symlink_metadata(&root)?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(crate::package_cache::error::CacheError::UnsafeCachePath(root).into());
            }
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(&root, permissions)?;
        }

        Ok(Self {
            root,
            locks: PackageLocks::new(),
        })
    }

    /// Return the default cache directory — Trả về thư mục cache mặc định.
    pub fn default_cache_dir() -> Result<PathBuf> {
        Ok(mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("web")
            .join("package-cache"))
    }

    /// Return a collision-resistant v2 directory for a package identifier.
    /// Trả về thư mục v2 chống va chạm cho package ID.
    pub(in crate::package_cache) fn package_dir(&self, package_id: &str) -> PathBuf {
        let key = blake3::hash(package_id.as_bytes()).to_hex().to_string();
        self.root.join("packages").join("npm").join("v2").join(key)
    }

    /// Get the package path (public for compatibility and inspection).
    /// Lấy path package (công khai để tương thích và kiểm tra).
    pub fn package_path(&self, package_id: &str) -> PathBuf {
        self.package_dir(package_id).join("package.tgz")
    }

    /// Return a safe old-format directory for a one-way compatibility read.
    /// Trả về thư mục định dạng cũ an toàn để đọc tương thích một chiều.
    pub(in crate::package_cache) fn legacy_package_dir(&self, package_id: &str) -> Option<PathBuf> {
        let component = package_id.replace(['/', '\\', ':'], "_");
        if !is_safe_legacy_component(&component) {
            return None;
        }
        Some(self.root.join("packages").join("npm").join(component))
    }

    /// Return the old package file path when the identifier is filesystem-safe.
    /// Trả path file package cũ nếu định danh an toàn với filesystem.
    pub(in crate::package_cache) fn legacy_package_path(
        &self,
        package_id: &str,
    ) -> Option<PathBuf> {
        self.legacy_package_dir(package_id)
            .map(|directory| directory.join("package.tgz"))
    }

    /// Create a cache rooted at a caller-supplied path — Tạo cache với root do caller cung cấp.
    #[doc(hidden)]
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            root,
            locks: PackageLocks::new(),
        }
    }
}

impl Default for PackageCache {
    fn default() -> Self {
        Self::new().expect("Failed to create default cache")
    }
}

fn is_safe_legacy_component(component: &str) -> bool {
    if component.is_empty()
        || component == "."
        || component == ".."
        || component.len() > MAX_LEGACY_COMPONENT_BYTES
        || component.ends_with('.')
        || component.ends_with(' ')
        || !component
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._@+-~".contains(&byte))
    {
        return false;
    }

    let device_stem = component
        .split(['.', '@'])
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    if matches!(device_stem.as_str(), "CON" | "PRN" | "AUX" | "NUL") {
        return false;
    }
    if let Some(suffix) = device_stem.strip_prefix("COM")
        && suffix.len() == 1
        && matches!(suffix.as_bytes()[0], b'1'..=b'9')
    {
        return false;
    }
    if let Some(suffix) = device_stem.strip_prefix("LPT")
        && suffix.len() == 1
        && matches!(suffix.as_bytes()[0], b'1'..=b'9')
    {
        return false;
    }
    true
}
