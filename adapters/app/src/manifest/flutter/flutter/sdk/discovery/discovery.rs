//! Discover Flutter SDK root from FLUTTER_ROOT or PATH without execution.
//! Dò root Flutter SDK từ FLUTTER_ROOT hoặc PATH mà không thực thi.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use std::path::PathBuf;

pub(crate) fn discover_flutter_sdk_root() -> FlutterResult<PathBuf> {
    if let Some(root) = std::env::var_os("FLUTTER_ROOT").filter(|value| !value.is_empty()) {
        return PathBuf::from(root)
            .canonicalize()
            .map_err(|error| FlutterError::Other(format!("resolve FLUTTER_ROOT: {error}")));
    }
    let path = std::env::var_os("PATH").ok_or_else(|| {
        FlutterError::Other("PATH is not set; configure FLUTTER_ROOT".to_string())
    })?;
    let pathext = std::env::var_os("PATHEXT");
    let executable = super::super::path_lookup::find_flutter_executable_in_path(
        &path,
        pathext.as_deref(),
        cfg!(windows),
    )
    .ok_or_else(|| {
        FlutterError::Other("find flutter on PATH; configure FLUTTER_ROOT".to_string())
    })?;
    for ancestor in executable.ancestors() {
        if ancestor.join("packages/flutter/pubspec.yaml").is_file() {
            return Ok(ancestor.to_path_buf());
        }
    }
    Err(FlutterError::Other(
        "could not locate Flutter SDK root from FLUTTER_ROOT or flutter on PATH".to_string(),
    ))
}
