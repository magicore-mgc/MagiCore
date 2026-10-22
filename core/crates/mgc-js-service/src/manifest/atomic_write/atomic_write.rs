//! Publish file bytes atomically with a safe create-new path.
//! Publish byte của file atomic với đường tạo mới an toàn.

use crate::manifest::error::ManifestError;
use mgc_types::{MgError, MgResult};
use std::io::Write;
use std::path::Path;

/// Atomically replace a regular file or safely publish a missing destination.
/// Thay atomic file thường hoặc publish an toàn khi đích chưa tồn tại.
pub fn atomic_write(path: &Path, data: &[u8]) -> MgResult<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => {
            mgc_adapter_base::project_file::atomic_write_regular(path, data)
        }
        Ok(_) => Err(ManifestError::UnsafeAtomicWriteDestination {
            path: path.to_path_buf(),
        }
        .into()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            atomic_create_new(path, data)
        }
        Err(source) => Err(ManifestError::InspectAtomicWriteDestination {
            path: path.to_path_buf(),
            source,
        }
        .into()),
    }
}

/// Publish a missing destination via a unique same-directory staging file.
/// Publish đích còn thiếu qua staging duy nhất cùng thư mục.
fn atomic_create_new(path: &Path, data: &[u8]) -> MgResult<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let file_name = path.file_name().ok_or_else(|| {
        MgError::from(ManifestError::InvalidAtomicWritePath {
            path: path.to_path_buf(),
        })
    })?;
    let temp = parent.join(format!(
        ".{}.mgc-tmp-{}-{}",
        file_name.to_string_lossy(),
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o644).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options
            .open(&temp)
            .map_err(|source| ManifestError::CreateAtomicStagingFile { source })?;
        file.write_all(data)
            .and_then(|()| file.sync_all())
            .map_err(|source| ManifestError::SyncAtomicStagingFile { source })?;
        match std::fs::symlink_metadata(path) {
            Ok(_) => {
                return Err(ManifestError::AtomicWriteDestinationAppeared {
                    path: path.to_path_buf(),
                });
            }
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
            Err(source) => {
                return Err(ManifestError::RecheckAtomicWriteDestination {
                    path: path.to_path_buf(),
                    source,
                });
            }
        }
        mgc_lockfile::atomic::atomic_replace_file(&temp, path).map_err(|source| {
            ManifestError::PublishAtomicFile {
                source: mgc_types::MgError::Io(source),
            }
        })?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|source| ManifestError::SyncAtomicWriteDirectory { source })?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result.map_err(Into::into)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
