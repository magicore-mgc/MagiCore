//! Local cache-layer path validation (no fetching).
//! Validate path layer cache cục bộ (không tải).

#[cfg(test)]
use super::super::error;
#[cfg(test)]
use crate::commands::template::{
    read_template_cache_version, template_cache_is_link, template_layer_has_contract,
    validate_template_identity,
};
#[cfg(test)]
use anyhow::Result;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::Path;

/// Validate any existing local layer path without fetching a missing layer.
/// Kiểm tra layer cache đã tồn tại mà không tải layer còn thiếu.
#[cfg(test)]
pub(crate) fn validate_cached_layer_path_in(
    rel: &str,
    configured_cache_root: &Path,
) -> Result<(), crate::scaffold::resolver::ScaffoldResolveError> {
    use crate::scaffold::resolver::ScaffoldResolveError;

    let (core, _) = rel.split_once('/').ok_or_else(|| {
        ScaffoldResolveError::Other(error::template_layer_identity_invalid().to_string())
    })?;
    validate_template_identity(core, rel)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;

    let cache_root = match fs::symlink_metadata(configured_cache_root) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            return Err(ScaffoldResolveError::Other(
                error::template_cache_unsafe_path(configured_cache_root).to_string(),
            ));
        }
        Ok(_) => configured_cache_root
            .canonicalize()
            .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(ScaffoldResolveError::Other(error.to_string())),
    };

    let mut path = cache_root.clone();
    for component in Path::new(rel).components() {
        let std::path::Component::Normal(component) = component else {
            return Err(ScaffoldResolveError::Other(
                error::template_layer_identity_invalid().to_string(),
            ));
        };
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                return Err(ScaffoldResolveError::Other(
                    error::template_cache_unsafe_path(&path).to_string(),
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(ScaffoldResolveError::Other(error.to_string())),
        }
    }

    template_layer_has_contract(&path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    read_template_cache_version(&path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    Ok(())
}
