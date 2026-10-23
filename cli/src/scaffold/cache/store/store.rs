//! Cache layer write, list and clear operations.
//! Thao tác ghi, liệt kê và xoá layer cache.

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::commands::template::{extract_tarball_into, lock_template_cache_layer};
use crate::scaffold::spec::ScaffoldSpec;

use super::super::entry::resolve_base;
use super::super::entry::{layer_key_for, validate_existing_directory, validated_entry};

pub(crate) fn write_at(
    root: &Path,
    spec: &ScaffoldSpec,
    version: &str,
    tarball: &[u8],
) -> Result<()> {
    semver::Version::parse(version)
        .map_err(|_| super::super::super::error::template_metadata_invalid())?;
    let base = resolve_base(root, spec, true)?
        .ok_or_else(|| super::super::super::error::template_cache_unsafe_path(root))?;
    let canonical_root = root.canonicalize()?;
    let _lock = lock_template_cache_layer(&canonical_root, &layer_key_for(spec))?;
    if validated_entry(&canonical_root, spec, version)?.is_some() {
        return Ok(());
    }
    fs::create_dir_all(&base)?;
    extract_tarball_into(tarball, &base.join(version), Some(version))
}

pub(crate) fn list_versions_at(root: &Path, spec: &ScaffoldSpec) -> Vec<String> {
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

pub(crate) fn clear_at(root: &Path, spec: &ScaffoldSpec, version: Option<&str>) -> Result<()> {
    if let Some(version) = version {
        semver::Version::parse(version)
            .map_err(|_| super::super::super::error::template_metadata_invalid())?;
    }
    let Some(base) = resolve_base(root, spec, false)? else {
        return Ok(());
    };
    let target = version.map_or(base.clone(), |version| base.join(version));
    if !validate_existing_directory(&target)? {
        return Ok(());
    }
    let canonical_root = root.canonicalize()?;
    let _lock = lock_template_cache_layer(&canonical_root, &layer_key_for(spec))?;
    validate_existing_directory(&target)?;
    remove_if_present(&target)
}

pub(crate) fn remove_if_present(path: &Path) -> Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
