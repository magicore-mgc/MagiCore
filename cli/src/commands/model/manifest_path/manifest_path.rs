// manifest_path owner — manifest_path owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::ManifestPathError;
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(in crate::commands::model) fn store_root() -> PathBuf {
    // Edition 2024 let-chain keeps the environment override check explicit.
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(root) = std::env::var("MAGICORE_STORE_ROOT")
        && !root.is_empty()
    {
        return PathBuf::from(root);
    }
    mgc_store::default_store_root()
}

pub(in crate::commands::model) fn model_manifest_dir() -> PathBuf {
    store_root().join("models")
}

pub(in crate::commands::model) fn model_manifest_path(name: &str) -> Result<PathBuf> {
    let dir = model_manifest_dir();
    Ok(dir.join(existing_or_encoded_manifest_path(&dir, name)?))
}

pub(in crate::commands::model) fn model_manifest_relative_path(name: &str) -> Result<PathBuf> {
    validate_model_name(name)?;
    let mut parts = name.split('/').collect::<Vec<_>>();
    let leaf = parts
        .pop()
        .ok_or_else(|| ManifestPathError::InvalidModelName(name.to_string()))?;
    let mut relative = PathBuf::new();
    for part in parts {
        relative.push(encode_model_path_segment(part));
    }
    relative.push(format!("{}.json", encode_model_path_segment(leaf)));
    Ok(relative)
}

pub(in crate::commands::model) fn existing_or_encoded_manifest_path(
    dir: &Path,
    name: &str,
) -> Result<PathBuf> {
    let encoded = model_manifest_relative_path(name)?;
    let mut legacy_parts = name.split('/').collect::<Vec<_>>();
    let leaf = legacy_parts
        .pop()
        .ok_or_else(|| ManifestPathError::InvalidModelName(name.to_string()))?;
    let mut legacy = PathBuf::new();
    for part in legacy_parts {
        legacy.push(part);
    }
    legacy.push(format!("{leaf}.json"));
    if legacy == encoded {
        return Ok(encoded);
    }
    match std::fs::symlink_metadata(dir.join(&legacy)) {
        Ok(metadata) if metadata.file_type().is_file() => Ok(legacy),
        Ok(_) => Err(ManifestPathError::UnsafeManifestPath.into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(encoded),
        Err(error) => Err(error.into()),
    }
}

fn encode_model_path_segment(segment: &str) -> String {
    let mut encoded = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.') {
            encoded.push(char::from(byte));
        } else {
            use std::fmt::Write;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

pub(in crate::commands::model) fn validate_model_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.contains(['\\', '\0'])
        || name
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
        || Path::new(name).is_absolute()
    {
        return Err(ManifestPathError::InvalidModelName(name.to_string()).into());
    }
    Ok(())
}
