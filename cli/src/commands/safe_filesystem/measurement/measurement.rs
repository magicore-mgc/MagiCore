//! Counts regular-file logical bytes without traversing symlink entries.
//! Đếm byte logic của file thường, không duyệt theo entry symlink.

use std::fs;
use std::path::Path;

use super::super::error::SafeFilesystemError;
use super::super::root::existing_directory_under;

pub(crate) fn logical_payload_bytes(root: &Path) -> Result<u64, SafeFilesystemError> {
    let canonical_root =
        fs::canonicalize(root).map_err(|source| SafeFilesystemError::CanonicalizeDirectory {
            path: display_path(root),
            source,
        })?;
    let root_metadata =
        fs::symlink_metadata(root).map_err(|source| SafeFilesystemError::InspectDirectory {
            path: display_path(root),
            source,
        })?;
    if root_metadata.file_type().is_symlink() {
        return Err(SafeFilesystemError::SymlinkedDirectory {
            path: display_path(root),
        });
    }
    if !root_metadata.is_dir() {
        return Err(SafeFilesystemError::NotDirectory {
            path: display_path(root),
        });
    }

    let mut total = 0u64;
    let mut pending = vec![canonical_root.clone()];
    while let Some(directory) = pending.pop() {
        let canonical_directory = fs::canonicalize(&directory).map_err(|source| {
            SafeFilesystemError::CanonicalizeDirectory {
                path: display_path(&directory),
                source,
            }
        })?;
        if !canonical_directory.starts_with(&canonical_root) {
            return Err(SafeFilesystemError::PathEscapesBoundary {
                path: display_path(&canonical_directory),
                boundary: display_path(&canonical_root),
            });
        }
        let directory_metadata = fs::symlink_metadata(&directory).map_err(|source| {
            SafeFilesystemError::InspectEntry {
                path: display_path(&directory),
                source,
            }
        })?;
        if directory_metadata.file_type().is_symlink() || !directory_metadata.is_dir() {
            return Err(SafeFilesystemError::EntryChangedType {
                path: display_path(&directory),
            });
        }

        let entries = fs::read_dir(&canonical_directory).map_err(|source| {
            SafeFilesystemError::ReadDirectory {
                path: display_path(&canonical_directory),
                source,
            }
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| SafeFilesystemError::ReadEntry {
                path: display_path(&canonical_directory),
                source,
            })?;
            let entry_path = entry.path();
            let file_type =
                entry
                    .file_type()
                    .map_err(|source| SafeFilesystemError::InspectEntry {
                        path: display_path(&entry_path),
                        source,
                    })?;
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                let Some(canonical_child) = existing_directory_under(&entry_path, &canonical_root)?
                else {
                    return Err(SafeFilesystemError::EntryChangedType {
                        path: display_path(&entry_path),
                    });
                };
                pending.push(canonical_child);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }

            let metadata = fs::symlink_metadata(&entry_path).map_err(|source| {
                SafeFilesystemError::InspectEntry {
                    path: display_path(&entry_path),
                    source,
                }
            })?;
            if metadata.file_type().is_symlink() {
                continue;
            }
            if !metadata.is_file() {
                return Err(SafeFilesystemError::EntryChangedType {
                    path: display_path(&entry_path),
                });
            }
            total = total.checked_add(metadata.len()).ok_or_else(|| {
                SafeFilesystemError::PayloadSizeOverflow {
                    path: display_path(&entry_path),
                }
            })?;
        }
    }
    Ok(total)
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
