use super::error;
use crate::commands::template::{MAX_TEMPLATE_VERSION_BYTES, template_cache_is_link};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub(crate) fn template_layer_has_contract(path: &Path) -> Result<bool> {
    for (entry, directory) in [
        (path.to_path_buf(), true),
        (path.join("template.toml"), false),
        (path.join("sources"), true),
    ] {
        let metadata = match fs::symlink_metadata(&entry) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if template_cache_is_link(&metadata)
            || (directory && !metadata.is_dir())
            || (!directory && !metadata.is_file())
        {
            return Err(error::template_cache_unsafe_path(&entry));
        }
    }
    crate::scaffold::processor::validate_template_manifest(path)?;
    Ok(true)
}

pub(crate) fn read_template_cache_version(path: &Path) -> Result<Option<String>> {
    let marker = path.join(".mgc-version");
    let metadata = match fs::symlink_metadata(&marker) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if template_cache_is_link(&metadata)
        || !metadata.is_file()
        || metadata.len() > MAX_TEMPLATE_VERSION_BYTES
    {
        return Err(error::template_cache_unsafe_path(&marker));
    }
    let version = fs::read_to_string(&marker)?.trim().to_string();
    semver::Version::parse(&version).map_err(|_| error::template_metadata_invalid())?;
    Ok(Some(version))
}
