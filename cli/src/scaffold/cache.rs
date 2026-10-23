//! Compatibility API for versioned scaffold archives; live CLI uses the processor-visible cache.
//! API cache version cũ giữ tương tác; không dùng directory/marker đơn lẻ làm bằng chứng cache hit.

use anyhow::Result;
use std::path::PathBuf;

use super::spec::ScaffoldSpec;

mod entry;
mod identity;
mod root;
mod store;

use entry::validated_entry;
use root::cache_root as resolve_cache_root;
#[cfg(test)]
pub(crate) use root::cache_root_with_override;
use store::{clear_at, list_versions_at, write_at};

pub struct ScaffoldCache;

impl ScaffoldCache {
    /// Construct a path only; filesystem methods validate the identity before accessing it.
    /// Hàm này không I/O; read/write/clear tự kiểm identity, không tin PathBuf do user ghép.
    pub fn path(spec: &ScaffoldSpec, version: &str) -> PathBuf {
        Self::cache_root()
            .join(spec.core.as_str())
            .join(&spec.normalized_name)
            .join(version)
    }

    pub fn has(spec: &ScaffoldSpec, version: &str) -> bool {
        validated_entry(&Self::cache_root(), spec, version).is_ok_and(|path| path.is_some())
    }

    /// Write an npm-style TAR.GZ layer with the same staged extractor as explicit fetch.
    /// Archive phải có một prefix chứa template.toml/sources; lỗi giữ nguyên cache cũ.
    pub fn write(spec: &ScaffoldSpec, version: &str, tarball: &[u8]) -> Result<()> {
        write_at(&Self::cache_root(), spec, version, tarball)
    }

    pub fn read(spec: &ScaffoldSpec, version: &str) -> Result<PathBuf> {
        validated_entry(&Self::cache_root(), spec, version)?.ok_or_else(|| {
            super::error::template_layer_missing_manifest(&format!(
                "{}/{}/{}",
                spec.core.as_str(),
                spec.normalized_name,
                version
            ))
        })
    }

    /// Return valid cache entries in semantic version order; invalid/incomplete entries are excluded.
    /// Chỉ liệt kê entry có contract và version marker khớp, không sort theo chuỗi.
    pub fn list_versions(spec: &ScaffoldSpec) -> Vec<String> {
        list_versions_at(&Self::cache_root(), spec)
    }

    pub fn clear(spec: &ScaffoldSpec, version: &str) -> Result<()> {
        clear_at(&Self::cache_root(), spec, Some(version))
    }

    pub fn clear_all(spec: &ScaffoldSpec) -> Result<()> {
        clear_at(&Self::cache_root(), spec, None)
    }

    fn cache_root() -> PathBuf {
        resolve_cache_root()
    }
}

#[cfg(test)]
#[path = "test/cache.rs"]
mod tests;
