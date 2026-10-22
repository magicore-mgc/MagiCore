//! Publish a metadata sidecar through unique durable staging — publish sidecar bằng staging bền vững.

use crate::cache::metadata::MetadataError;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(in crate::cache::metadata) fn publish_metadata(
    destination: &Path,
    bytes: &[u8],
) -> Result<(), MetadataError> {
    publish_metadata_with_replace(
        destination,
        bytes,
        mgc_platform::atomic_replace::atomic_replace_file,
    )
}

pub(super) fn publish_metadata_with_replace(
    destination: &Path,
    bytes: &[u8],
    replace: impl FnOnce(&Path, &Path) -> std::io::Result<()>,
) -> Result<(), MetadataError> {
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if destination.file_name().is_none() {
        return Err(MetadataError::InvalidSidecarPath);
    }
    let original_permissions = destination_permissions(destination)?;
    let staging = staging_path(parent);
    let mut staging_created = false;

    let result = (|| {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&staging)
            .map_err(|source| MetadataError::io("create metadata staging file", source))?;
        staging_created = true;
        file.write_all(bytes)
            .map_err(|source| MetadataError::io("write metadata staging file", source))?;
        if let Some(permissions) = original_permissions {
            file.set_permissions(permissions)
                .map_err(|source| MetadataError::io("preserve metadata permissions", source))?;
        }
        file.sync_all()
            .map_err(|source| MetadataError::io("sync metadata staging file", source))?;
        drop(file);

        // Recheck the sidecar. Atomic replacement publishes the directory entry and never opens
        // a destination symlink for truncation. A swap after this check is still replaced as an entry.
        let _ = destination_permissions(destination)?;
        replace(&staging, destination)
            .map_err(|source| MetadataError::io("atomically replace metadata sidecar", source))?;
        #[cfg(unix)]
        fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|source| MetadataError::io("sync metadata directory", source))?;
        Ok(())
    })();

    if result.is_err() && staging_created {
        let _ = fs::remove_file(&staging);
    }
    result
}

fn destination_permissions(destination: &Path) -> Result<Option<fs::Permissions>, MetadataError> {
    match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(MetadataError::SymlinkSidecar),
        Ok(metadata) if metadata.file_type().is_file() => Ok(Some(metadata.permissions())),
        Ok(_) => Err(MetadataError::NonRegularSidecar),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(MetadataError::io("inspect metadata destination", source)),
    }
}

fn staging_path(parent: &Path) -> PathBuf {
    parent.join(format!(
        ".mgc-ai-metadata-{}-{}.tmp",
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}
