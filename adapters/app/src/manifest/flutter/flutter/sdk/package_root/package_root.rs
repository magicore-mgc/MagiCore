//! Validate Flutter SDK package names and canonical package roots.
//! Kiểm tra tên và root canonical package Flutter SDK.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use std::path::{Path, PathBuf};

pub(crate) fn flutter_sdk_package_root(sdk_root: &Path, name: &str) -> FlutterResult<PathBuf> {
    let candidate = if name == "sky_engine" {
        sdk_root.join("bin/cache/pkg/sky_engine")
    } else {
        sdk_root.join("packages").join(name)
    };
    let canonical = candidate.canonicalize().map_err(|error| {
        FlutterError::Other(format!(
            "Flutter SDK package '{name}' is unavailable at '{}': {error}",
            candidate.display()
        ))
    })?;
    let canonical_root = sdk_root
        .canonicalize()
        .map_err(|error| FlutterError::Other(format!("canonicalize Flutter SDK root: {error}")))?;
    if !canonical.starts_with(&canonical_root) || !canonical.is_dir() {
        return Err(FlutterError::Integrity(format!(
            "Flutter SDK package '{name}' resolves outside the selected SDK"
        )));
    }
    Ok(canonical)
}

pub(crate) fn validate_flutter_sdk_package_name(name: &str) -> FlutterResult<()> {
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(FlutterError::Integrity(format!(
            "invalid Flutter SDK package name '{name}'"
        )));
    }
    Ok(())
}
