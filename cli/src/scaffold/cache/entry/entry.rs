//! Cache entry resolution and contract validation.
//! Resolve entry cache và validate contract.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::template::{
    read_template_cache_version, template_cache_is_link, template_layer_has_contract,
};
use crate::scaffold::spec::ScaffoldSpec;

use super::super::identity::validate_identity;

fn layer_key(spec: &ScaffoldSpec) -> String {
    format!("{}/{}", spec.core.as_str(), spec.normalized_name)
}

pub(crate) fn validate_existing_directory(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            Err(super::super::super::error::template_cache_unsafe_path(path))
        }
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

pub(crate) fn resolve_base(
    root: &Path,
    spec: &ScaffoldSpec,
    create: bool,
) -> Result<Option<PathBuf>> {
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

pub(crate) fn validated_entry(
    root: &Path,
    spec: &ScaffoldSpec,
    version: &str,
) -> Result<Option<PathBuf>> {
    semver::Version::parse(version)
        .map_err(|_| super::super::super::error::template_metadata_invalid())?;
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

/// Cache layer key used for per-layer locking.
pub(crate) fn layer_key_for(spec: &ScaffoldSpec) -> String {
    layer_key(spec)
}
