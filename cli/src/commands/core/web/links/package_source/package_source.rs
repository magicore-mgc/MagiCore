//! Resolve an explicit local, project-local or MagiCore global package source.
//! Tìm source package theo thứ tự path rõ ràng, project và global MagiCore.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::error::WebLinkError;
use super::global_roots::magicore_global_package_roots;

pub(in crate::commands::core::web::links) fn find_package_source(
    root: &Path,
    package: &str,
) -> Result<PathBuf> {
    let is_absolute = Path::new(package).is_absolute() || package.starts_with('/');
    if package.starts_with('.') || is_absolute {
        let path = if is_absolute {
            PathBuf::from(package)
        } else {
            root.join(package)
        };
        if path.exists() {
            return path
                .canonicalize()
                .map_err(WebLinkError::filesystem)
                .map_err(anyhow::Error::new);
        }
        return Err(WebLinkError::local_path_not_found(&path).into());
    }

    let local = root.join("node_modules").join(package);
    if local.exists() {
        return Ok(local);
    }

    for global in magicore_global_package_roots() {
        let global_package = global.join(package);
        if global_package.exists() {
            return Ok(global_package);
        }
    }

    Err(WebLinkError::package_not_found(package).into())
}
