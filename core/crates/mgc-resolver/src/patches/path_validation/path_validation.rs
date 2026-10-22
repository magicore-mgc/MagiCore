//! Validate patch-relative paths and contain resolved targets in the vstore.
//! Kiểm tra đường dẫn tương đối và giữ đích đã phân giải bên trong vstore.

use std::path::{Component, Path, PathBuf};

use super::error::PathValidationError;

pub(in crate::patches) fn validate_patch_relative_path(
    path: &str,
) -> Result<(), PathValidationError> {
    if path.is_empty() || path.contains('\\') || path.contains('\0') {
        return Err(PathValidationError::InvalidRelativeForm);
    }
    let candidate = Path::new(path);
    if candidate.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_) | Component::CurDir
        )
    }) {
        return Err(PathValidationError::Traversal(path.to_string()));
    }
    Ok(())
}

pub(in crate::patches) fn safe_vstore_target(
    vstore_root: &Path,
    relative_path: &str,
) -> Result<PathBuf, PathValidationError> {
    validate_patch_relative_path(relative_path)?;
    let root = vstore_root.canonicalize()?;
    let target = root.join(relative_path);
    let canonical_target = target.canonicalize()?;
    if !canonical_target.starts_with(&root) {
        return Err(PathValidationError::EscapesStore(
            target.display().to_string(),
        ));
    }
    if !canonical_target.is_file() {
        return Err(PathValidationError::NotRegularFile(
            target.display().to_string(),
        ));
    }
    Ok(canonical_target)
}

#[cfg(test)]
#[path = "test/path_validation.rs"]
mod tests;
