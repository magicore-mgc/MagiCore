//! Open digest-addressed artifact files safely and verify their SHA-256.
//! Mở file artifact theo digest an toàn và xác minh SHA-256.

use std::{io::Read, path::Path};

use super::error::PythonArtifactCacheError;

pub(in crate::install) fn read_cached_python_artifact(
    path: &Path,
    expected_sha256: &str,
) -> Result<Option<Vec<u8>>, PythonArtifactCacheError> {
    use sha2::Digest;

    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => {
            return Err(PythonArtifactCacheError::NotRegularFile {
                path: path.display().to_string(),
            });
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(PythonArtifactCacheError::InspectPath {
                path: path.display().to_string(),
                source,
            });
        }
    }

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let mut file = options
        .open(path)
        .map_err(|source| PythonArtifactCacheError::OpenNoFollow {
            path: path.display().to_string(),
            source,
        })?;
    let metadata =
        file.metadata()
            .map_err(|source| PythonArtifactCacheError::InspectOpenedFile {
                path: path.display().to_string(),
                source,
            })?;
    if !metadata.is_file() {
        return Err(PythonArtifactCacheError::NotRegularFile {
            path: path.display().to_string(),
        });
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(PythonArtifactCacheError::ReparsePoint {
                path: path.display().to_string(),
            });
        }
    }

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|source| PythonArtifactCacheError::ReadFile {
            path: path.display().to_string(),
            source,
        })?;
    let actual = hex::encode(sha2::Sha256::digest(&bytes));
    if !actual.eq_ignore_ascii_case(expected_sha256) {
        return Err(PythonArtifactCacheError::DigestMismatch {
            path: path.display().to_string(),
            expected: expected_sha256.to_string(),
            actual,
        });
    }
    Ok(Some(bytes))
}
