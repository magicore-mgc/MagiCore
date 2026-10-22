//! Replaces the virtual store only after validating and preparing the destination ancestry.
//! Chỉ thay virtual store sau khi xác thực và chuẩn bị ancestry đích.

use std::path::Path;

use super::error::VirtualStoreRestoreError;
use crate::commands::store::backup_storage::{
    clear_dir, copy_validated_tree, prepare_destination_root, validate_present_tree,
};

pub(in crate::commands::store::restore) fn restore_virtual_store(
    source: &Path,
    destination: &Path,
) -> Result<(), VirtualStoreRestoreError> {
    let validated_source = validate_present_tree(source).map_err(VirtualStoreRestoreError::Copy)?;
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| VirtualStoreRestoreError::InvalidDestination(destination.to_path_buf()))?;
    let file_name = destination
        .file_name()
        .ok_or_else(|| VirtualStoreRestoreError::InvalidDestination(destination.to_path_buf()))?;
    let canonical_parent = prepare_destination_root(parent).map_err(|source| {
        VirtualStoreRestoreError::PrepareParent {
            path: parent.to_path_buf(),
            source,
        }
    })?;
    let canonical_destination = canonical_parent.join(file_name);
    clear_dir(&canonical_destination).map_err(VirtualStoreRestoreError::Clear)?;
    copy_validated_tree(&validated_source, &canonical_destination)
        .map_err(VirtualStoreRestoreError::Copy)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
