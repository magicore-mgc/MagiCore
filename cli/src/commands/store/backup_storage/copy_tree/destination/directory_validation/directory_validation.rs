//! Validate contained child and empty destination directories.
//! Xác thực directory con nằm trong root và destination rỗng.

use std::fs;
use std::path::Path;

use super::super::error::DestinationError;

pub(in crate::commands::store) fn ensure_destination_directory(
    path: &Path,
    root: &Path,
) -> Result<(), DestinationError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(DestinationError::SymlinkDirectory(path.to_path_buf()));
        }
        Ok(metadata) if !metadata.is_dir() => {
            return Err(DestinationError::NotRealDirectory(path.to_path_buf()));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|source| DestinationError::CreateDirectory {
                path: path.to_path_buf(),
                source,
            })?;
        }
        Err(source) => {
            return Err(DestinationError::InspectDirectory {
                path: path.to_path_buf(),
                source,
            });
        }
    }
    let canonical =
        fs::canonicalize(path).map_err(|source| DestinationError::CanonicalizeDirectory {
            path: path.to_path_buf(),
            source,
        })?;
    if !canonical.starts_with(root) {
        return Err(DestinationError::DirectoryEscapesRoot {
            path: canonical,
            root: root.to_path_buf(),
        });
    }
    Ok(())
}

pub(in crate::commands::store) fn ensure_empty_destination_directory(
    path: &Path,
) -> Result<(), DestinationError> {
    let metadata =
        fs::symlink_metadata(path).map_err(|source| DestinationError::InspectDirectory {
            path: path.to_path_buf(),
            source,
        })?;
    if metadata.file_type().is_symlink() {
        return Err(DestinationError::SymlinkDirectory(path.to_path_buf()));
    }
    if !metadata.is_dir() {
        return Err(DestinationError::NotRealDirectory(path.to_path_buf()));
    }
    let mut entries = fs::read_dir(path).map_err(|source| DestinationError::InspectDirectory {
        path: path.to_path_buf(),
        source,
    })?;
    match entries.next() {
        Some(Ok(_)) => Err(DestinationError::DestinationNotEmpty(path.to_path_buf())),
        Some(Err(source)) => Err(DestinationError::InspectDirectory {
            path: path.to_path_buf(),
            source,
        }),
        None => Ok(()),
    }
}
