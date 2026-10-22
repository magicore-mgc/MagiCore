//! Inspects the configured store without creating its path.
//! Kiểm tra store đã cấu hình mà không tạo đường dẫn store.

use std::{
    fs,
    path::{Path, PathBuf},
};

use super::StorageError;

pub(in crate::commands::doctor) struct StoreSnapshot {
    pub(in crate::commands::doctor) directory: PathBuf,
    pub(in crate::commands::doctor) writable: bool,
    pub(in crate::commands::doctor) free_bytes: u64,
    pub(in crate::commands::doctor) volume_path: Option<PathBuf>,
    pub(in crate::commands::doctor) status: StoreStatus,
    pub(in crate::commands::doctor) free_space_error: Option<StorageError>,
}

pub(in crate::commands::doctor) enum StoreStatus {
    Missing,
    BrokenSymlink { path: PathBuf },
    Writable,
    NotDirectory,
    NotWritable(StorageError),
    InspectionFailed(StorageError),
}

impl StoreStatus {
    pub(in crate::commands::doctor) fn description(&self) -> String {
        match self {
            Self::Missing => "configured store directory is missing".to_string(),
            Self::BrokenSymlink { path } => format!(
                "configured store path contains a symbolic link with a missing target at '{}'",
                path.display()
            ),
            Self::Writable => "configured store directory is writable".to_string(),
            Self::NotDirectory => "configured store path exists but is not a directory".to_string(),
            Self::NotWritable(error) | Self::InspectionFailed(error) => error.to_string(),
        }
    }
}

pub(in crate::commands::doctor) fn inspect() -> StoreSnapshot {
    inspect_at(mgc_store::default_store_root())
}

pub(in crate::commands::doctor) fn inspect_at(directory: PathBuf) -> StoreSnapshot {
    let status = inspect_status(&directory);
    let writable = matches!(&status, StoreStatus::Writable);
    let (free_bytes, free_space_error, volume_path) = fs_avail(&directory);
    StoreSnapshot {
        directory,
        writable,
        free_bytes,
        volume_path,
        status,
        free_space_error,
    }
}

fn inspect_status(directory: &Path) -> StoreStatus {
    let link_metadata = match fs::symlink_metadata(directory) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return match dangling_symlink_ancestor(directory) {
                Some(path) => StoreStatus::BrokenSymlink { path },
                None => StoreStatus::Missing,
            };
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotADirectory => {
            return StoreStatus::NotDirectory;
        }
        Err(source) => {
            return StoreStatus::InspectionFailed(StorageError::InspectPath {
                path: directory.to_path_buf(),
                source,
            });
        }
        Ok(metadata) => metadata,
    };
    let metadata = if link_metadata.file_type().is_symlink() {
        match fs::metadata(directory) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return StoreStatus::BrokenSymlink {
                    path: directory.to_path_buf(),
                };
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotADirectory => {
                return StoreStatus::NotDirectory;
            }
            Err(source) => {
                return StoreStatus::InspectionFailed(StorageError::InspectPath {
                    path: directory.to_path_buf(),
                    source,
                });
            }
            Ok(metadata) => metadata,
        }
    } else {
        link_metadata
    };
    if !metadata.is_dir() {
        StoreStatus::NotDirectory
    } else {
        match tempfile::Builder::new()
            .prefix(".mgc-doctor-write-probe-")
            .tempfile_in(directory)
        {
            Err(source) => StoreStatus::NotWritable(StorageError::CreateWriteProbe {
                directory: directory.to_path_buf(),
                source,
            }),
            Ok(probe) => {
                let path = probe.path().to_path_buf();
                match probe.close() {
                    Ok(()) => StoreStatus::Writable,
                    Err(source) => {
                        StoreStatus::NotWritable(StorageError::RemoveWriteProbe { path, source })
                    }
                }
            }
        }
    }
}

fn dangling_symlink_ancestor(path: &Path) -> Option<PathBuf> {
    let mut candidate = Some(path);
    while let Some(current) = candidate {
        let link_metadata = fs::symlink_metadata(current).ok();
        if link_metadata.is_some_and(|metadata| metadata.file_type().is_symlink())
            && fs::metadata(current)
                .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        {
            return Some(current.to_path_buf());
        }
        candidate = current.parent();
    }
    None
}

fn fs_avail(store_path: &Path) -> (u64, Option<StorageError>, Option<PathBuf>) {
    let Some(volume_path) = nearest_existing_directory(store_path) else {
        return (
            0,
            Some(StorageError::NoVolumeAncestor {
                path: store_path.to_path_buf(),
            }),
            None,
        );
    };
    match fs2::available_space(&volume_path) {
        Ok(bytes) => (bytes, None, Some(volume_path)),
        Err(source) => (
            0,
            Some(StorageError::AvailableSpace {
                path: volume_path.clone(),
                source,
            }),
            Some(volume_path),
        ),
    }
}

pub(in crate::commands::doctor) fn nearest_existing_directory(path: &Path) -> Option<PathBuf> {
    let mut candidate = Some(path);
    while let Some(current) = candidate {
        if fs::metadata(current).is_ok_and(|metadata| metadata.is_dir()) {
            return Some(current.to_path_buf());
        }
        candidate = current.parent();
    }
    None
}
