//! Validates the project and virtual-store directory chain.
//! Xác thực project và chuỗi thư mục virtual store.

use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::safe_filesystem::{canonical_directory, existing_directory_under};

use super::super::error::VirtualStoreCleanupError;

pub(in crate::commands::dedupe::virtual_store_cleanup) fn existing_store_root(
    project_root: &Path,
) -> Result<Option<PathBuf>, VirtualStoreCleanupError> {
    let canonical_project = fs::canonicalize(project_root).map_err(|source| {
        VirtualStoreCleanupError::CanonicalizeProjectRoot {
            path: display_path(project_root),
            source,
        }
    })?;
    let canonical_project = canonical_directory(&canonical_project)?;

    let Some(node_modules) =
        existing_directory_under(&canonical_project.join("node_modules"), &canonical_project)?
    else {
        return Ok(None);
    };

    Ok(existing_directory_under(
        &node_modules.join(".magicore"),
        &node_modules,
    )?)
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
