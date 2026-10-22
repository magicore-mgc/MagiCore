//! Copies a no-follow regular source file through a same-directory staging file.
//! Copy file source thường no-follow qua file staging cùng thư mục.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::error::FileCopyError;

static NEXT_STAGING_ID: AtomicU64 = AtomicU64::new(0);

pub(crate) fn copy_regular_file_no_follow(
    source: &Path,
    destination: &Path,
) -> Result<(), FileCopyError> {
    let mut source_file =
        mgc_platform::open_regular_file_no_follow(source).map_err(|source_error| {
            FileCopyError::OpenSource {
                path: source.to_path_buf(),
                source: source_error,
            }
        })?;
    let destination_path = canonical_destination(destination)?;
    validate_destination(&destination_path)?;
    let (staging_path, mut staging_file) = create_staging_file(&destination_path)?;
    let mut staging = StagingGuard::new(staging_path.clone());

    io::copy(&mut source_file, &mut staging_file).map_err(|source_error| FileCopyError::Copy {
        source_path: source.to_path_buf(),
        destination: destination_path.clone(),
        source: source_error,
    })?;
    let source_permissions = source_file
        .metadata()
        .map_err(|source_error| FileCopyError::Copy {
            source_path: source.to_path_buf(),
            destination: destination_path.clone(),
            source: source_error,
        })?
        .permissions();
    staging_file
        .set_permissions(source_permissions)
        .map_err(|source_error| FileCopyError::SetPermissions {
            destination: destination_path.clone(),
            source: source_error,
        })?;
    staging_file
        .sync_all()
        .map_err(|source_error| FileCopyError::Sync {
            destination: destination_path.clone(),
            source: source_error,
        })?;
    drop(staging_file);

    validate_destination(&destination_path)?;
    mgc_platform::atomic_replace::atomic_replace_file(&staging_path, &destination_path).map_err(
        |source_error| FileCopyError::Publish {
            destination: destination_path,
            source: source_error,
        },
    )?;
    staging.disarm();
    Ok(())
}

fn canonical_destination(destination: &Path) -> Result<PathBuf, FileCopyError> {
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = destination
        .file_name()
        .ok_or_else(|| FileCopyError::InvalidDestination(destination.to_path_buf()))?;
    let canonical_parent =
        fs::canonicalize(parent).map_err(|source| FileCopyError::CanonicalizeParent {
            path: parent.to_path_buf(),
            source,
        })?;
    if !fs::metadata(&canonical_parent)
        .map_err(|source| FileCopyError::CanonicalizeParent {
            path: parent.to_path_buf(),
            source,
        })?
        .is_dir()
    {
        return Err(FileCopyError::ParentNotDirectory(parent.to_path_buf()));
    }
    Ok(canonical_parent.join(file_name))
}

fn validate_destination(destination: &Path) -> Result<(), FileCopyError> {
    match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(FileCopyError::SymlinkDestination(destination.to_path_buf()))
        }
        Ok(metadata) if !metadata.is_file() => Err(FileCopyError::NotRegularDestination(
            destination.to_path_buf(),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(FileCopyError::InspectDestination {
            path: destination.to_path_buf(),
            source,
        }),
    }
}

fn create_staging_file(destination: &Path) -> Result<(PathBuf, File), FileCopyError> {
    let parent = destination
        .parent()
        .ok_or_else(|| FileCopyError::InvalidDestination(destination.to_path_buf()))?;
    for _ in 0..64 {
        let id = NEXT_STAGING_ID.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(".mgc-copy-{}-{id}.tmp", std::process::id()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => return Ok((candidate, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(source) => {
                return Err(FileCopyError::CreateStaging {
                    destination: destination.to_path_buf(),
                    source,
                });
            }
        }
    }
    Err(FileCopyError::ExhaustedStagingNames(
        destination.to_path_buf(),
    ))
}

struct StagingGuard {
    path: PathBuf,
    armed: bool,
}

impl StagingGuard {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for StagingGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
