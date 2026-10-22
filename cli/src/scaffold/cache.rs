//! Compatibility API for versioned scaffold archives; live CLI uses the processor-visible cache.
//! API cache version cũ giữ tương thích; không dùng directory/marker đơn lẻ làm bằng chứng cache hit.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::spec::ScaffoldSpec;
use crate::commands::template::{
    extract_tarball_into, lock_template_cache_layer, read_template_cache_version,
    template_cache_is_link, template_layer_has_contract,
};

fn cache_root_with_override(override_root: Option<PathBuf>, default_root: PathBuf) -> PathBuf {
    override_root.unwrap_or(default_root)
}

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
        let default_root = mgc_platform::paths::mgc_data_dir().join("scaffolds");
        let override_root = std::env::var_os("MGC_SCAFFOLDS_DIR").map(PathBuf::from);
        cache_root_with_override(override_root, default_root)
    }
}

fn layer_key(spec: &ScaffoldSpec) -> String {
    format!("{}/{}", spec.core.as_str(), spec.normalized_name)
}

fn validate_identity(spec: &ScaffoldSpec) -> Result<()> {
    let name = &spec.normalized_name;
    if name.is_empty()
        || name.contains(['\\', ':'])
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|part| part.is_empty() || part.ends_with(['.', ' ']))
    {
        return Err(super::error::template_layer_identity_invalid());
    }
    Ok(())
}

fn validate_existing_directory(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            Err(super::error::template_cache_unsafe_path(path))
        }
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn resolve_base(root: &Path, spec: &ScaffoldSpec, create: bool) -> Result<Option<PathBuf>> {
    validate_identity(spec)?;
    if create {
        fs::create_dir_all(root)?;
    }
    if !validate_existing_directory(root)? {
        return Ok(None);
    }
    let mut base = root.canonicalize()?;
    for part in std::iter::once(spec.core.as_str()).chain(spec.normalized_name.split('/')) {
        base.push(part);
        validate_existing_directory(&base)?;
    }
    Ok(Some(base))
}

fn validated_entry(root: &Path, spec: &ScaffoldSpec, version: &str) -> Result<Option<PathBuf>> {
    semver::Version::parse(version).map_err(|_| super::error::template_metadata_invalid())?;
    let Some(base) = resolve_base(root, spec, false)? else {
        return Ok(None);
    };
    let target = base.join(version);
    if !validate_existing_directory(&target)?
        || !template_layer_has_contract(&target)?
        || read_template_cache_version(&target)?.as_deref() != Some(version)
    {
        return Ok(None);
    }
    Ok(Some(target))
}

fn write_at(root: &Path, spec: &ScaffoldSpec, version: &str, tarball: &[u8]) -> Result<()> {
    semver::Version::parse(version).map_err(|_| super::error::template_metadata_invalid())?;
    let base = resolve_base(root, spec, true)?
        .ok_or_else(|| super::error::template_cache_unsafe_path(root))?;
    let canonical_root = root.canonicalize()?;
    let _lock = lock_template_cache_layer(&canonical_root, &layer_key(spec))?;
    if validated_entry(&canonical_root, spec, version)?.is_some() {
        return Ok(());
    }
    fs::create_dir_all(&base)?;
    extract_tarball_into(tarball, &base.join(version), Some(version))
}

fn list_versions_at(root: &Path, spec: &ScaffoldSpec) -> Vec<String> {
    let Ok(Some(base)) = resolve_base(root, spec, false) else {
        return vec![];
    };
    let Ok(entries) = fs::read_dir(base) else {
        return vec![];
    };
    let mut versions = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().into_string().ok()?;
            let parsed = semver::Version::parse(&name).ok()?;
            validated_entry(root, spec, &name).ok().flatten()?;
            Some((parsed, name))
        })
        .collect::<Vec<_>>();
    versions.sort_by(|a, b| b.0.cmp(&a.0));
    versions.into_iter().map(|(_, name)| name).collect()
}

fn clear_at(root: &Path, spec: &ScaffoldSpec, version: Option<&str>) -> Result<()> {
    if let Some(version) = version {
        semver::Version::parse(version).map_err(|_| super::error::template_metadata_invalid())?;
    }
    let Some(base) = resolve_base(root, spec, false)? else {
        return Ok(());
    };
    let target = version.map_or(base.clone(), |version| base.join(version));
    if !validate_existing_directory(&target)? {
        return Ok(());
    }
    let canonical_root = root.canonicalize()?;
    let _lock = lock_template_cache_layer(&canonical_root, &layer_key(spec))?;
    validate_existing_directory(&target)?;
    remove_if_present(&target)
}

fn remove_if_present(path: &Path) -> Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
#[path = "test/cache.rs"]
mod tests;
