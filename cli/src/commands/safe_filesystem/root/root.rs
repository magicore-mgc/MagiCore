//! Validates real directory roots and confines children to canonical parents.
//! Xác thực root thư mục thật và giới hạn con bên trong parent canonical.

use std::fs;
use std::path::{Path, PathBuf};

use super::super::error::SafeFilesystemError;

/// Canonicalize an existing directory after checking its final component without following links.
/// Chuẩn hóa thư mục có sẵn sau khi kiểm tra component cuối mà không follow link.
pub(crate) fn canonical_directory(path: &Path) -> Result<PathBuf, SafeFilesystemError> {
    let metadata =
        fs::symlink_metadata(path).map_err(|source| SafeFilesystemError::InspectDirectory {
            path: display_path(path),
            source,
        })?;
    if metadata.file_type().is_symlink() {
        return Err(SafeFilesystemError::SymlinkedDirectory {
            path: display_path(path),
        });
    }
    if !metadata.is_dir() {
        return Err(SafeFilesystemError::NotDirectory {
            path: display_path(path),
        });
    }

    let canonical =
        fs::canonicalize(path).map_err(|source| SafeFilesystemError::CanonicalizeDirectory {
            path: display_path(path),
            source,
        })?;
    let confirmed =
        fs::symlink_metadata(path).map_err(|source| SafeFilesystemError::InspectDirectory {
            path: display_path(path),
            source,
        })?;
    if confirmed.file_type().is_symlink() {
        return Err(SafeFilesystemError::SymlinkedDirectory {
            path: display_path(path),
        });
    }
    if !confirmed.is_dir() {
        return Err(SafeFilesystemError::NotDirectory {
            path: display_path(path),
        });
    }
    Ok(canonical)
}

/// Return a canonical real child directory; a missing child is `Ok(None)`.
/// Trả về thư mục con thật canonical; thư mục chưa có là `Ok(None)`.
pub(crate) fn existing_directory_under(
    path: &Path,
    expected_parent: &Path,
) -> Result<Option<PathBuf>, SafeFilesystemError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(SafeFilesystemError::InspectDirectory {
                path: display_path(path),
                source,
            });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(SafeFilesystemError::SymlinkedDirectory {
            path: display_path(path),
        });
    }
    if !metadata.is_dir() {
        return Err(SafeFilesystemError::NotDirectory {
            path: display_path(path),
        });
    }

    let canonical =
        fs::canonicalize(path).map_err(|source| SafeFilesystemError::CanonicalizeDirectory {
            path: display_path(path),
            source,
        })?;
    if !canonical.starts_with(expected_parent) {
        return Err(SafeFilesystemError::DirectoryEscapesParent {
            path: display_path(&canonical),
            parent: display_path(expected_parent),
        });
    }

    // Recheck without following after canonicalization; this path-based seam cannot close a
    // hostile concurrent replacement race.
    // Kiểm tra lại không follow sau canonicalize; seam path-based chưa chặn được race swap đồng thời.
    let confirmed =
        fs::symlink_metadata(path).map_err(|source| SafeFilesystemError::InspectDirectory {
            path: display_path(path),
            source,
        })?;
    if confirmed.file_type().is_symlink() {
        return Err(SafeFilesystemError::SymlinkedDirectory {
            path: display_path(path),
        });
    }
    if !confirmed.is_dir() {
        return Err(SafeFilesystemError::NotDirectory {
            path: display_path(path),
        });
    }
    Ok(Some(canonical))
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
