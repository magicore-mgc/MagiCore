//! Publishes a restored database through a same-directory staged no-follow copy.
//! Publish database restore qua copy no-follow staging cùng thư mục.

use std::path::Path;

use super::error::DatabaseRestoreError;
use crate::commands::safe_filesystem::copy_regular_file_no_follow;
use crate::commands::store::backup_storage::prepare_destination_root;

pub(in crate::commands::store::restore) fn restore_database(
    source: &Path,
    destination: &Path,
) -> Result<(), DatabaseRestoreError> {
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| DatabaseRestoreError::InvalidDestination(destination.to_path_buf()))?;
    let file_name = destination
        .file_name()
        .ok_or_else(|| DatabaseRestoreError::InvalidDestination(destination.to_path_buf()))?;
    let canonical_parent =
        prepare_destination_root(parent).map_err(|source| DatabaseRestoreError::PrepareParent {
            path: parent.to_path_buf(),
            source,
        })?;
    let canonical_destination = canonical_parent.join(file_name);
    copy_regular_file_no_follow(source, &canonical_destination).map_err(|source_error| {
        DatabaseRestoreError::Copy {
            source_path: source.to_path_buf(),
            destination: canonical_destination,
            source: source_error,
        }
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
