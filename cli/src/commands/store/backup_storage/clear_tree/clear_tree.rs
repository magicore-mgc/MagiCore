//! Clears direct children without following a symlinked root or link entry.
//! Xóa child trực tiếp, không follow root hay entry symlink.

use super::error::ClearTreeError;
use std::fs;
use std::path::Path;

pub(in crate::commands::store) fn clear_dir(destination: &Path) -> anyhow::Result<()> {
    clear_tree(destination).map_err(anyhow::Error::new)
}

fn clear_tree(destination: &Path) -> Result<(), ClearTreeError> {
    let root_metadata = match fs::symlink_metadata(destination) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(ClearTreeError::InspectRoot {
                path: destination.to_path_buf(),
                source,
            });
        }
    };
    if root_metadata.file_type().is_symlink() {
        return Err(ClearTreeError::SymlinkRoot(destination.to_path_buf()));
    }
    if !root_metadata.is_dir() {
        return Err(ClearTreeError::NotDirectory(destination.to_path_buf()));
    }

    for entry in fs::read_dir(destination).map_err(|source| ClearTreeError::ReadDirectory {
        path: destination.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| ClearTreeError::ReadEntry {
            path: destination.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let metadata =
            fs::symlink_metadata(&path).map_err(|source| ClearTreeError::InspectEntry {
                path: path.clone(),
                source,
            })?;
        if metadata.file_type().is_symlink() {
            remove_link(&path, &metadata)?;
        } else if metadata.is_dir() {
            fs::remove_dir_all(&path).map_err(|source| ClearTreeError::RemoveDirectory {
                path: path.clone(),
                source,
            })?;
        } else {
            fs::remove_file(&path).map_err(|source| ClearTreeError::RemoveFile {
                path: path.clone(),
                source,
            })?;
        }
    }
    Ok(())
}

fn remove_link(path: &Path, _metadata: &fs::Metadata) -> Result<(), ClearTreeError> {
    #[cfg(unix)]
    let result = fs::remove_file(path);
    #[cfg(windows)]
    let result = {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x0000_0010;
        if _metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0 {
            fs::remove_dir(path)
        } else {
            fs::remove_file(path)
        }
    };
    #[cfg(not(any(unix, windows)))]
    let result = fs::remove_file(path);
    result.map_err(|source| ClearTreeError::RemoveLink {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
