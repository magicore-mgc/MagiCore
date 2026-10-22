//! Creates only a missing store and reports the verified result.
//! Chỉ tạo store đang thiếu và trả kết quả đã được kiểm tra.

#[cfg(test)]
use std::path::PathBuf;
use std::{fs, io, path::Path};

use serde::Serialize;

use super::super::{StorageError, StoreSnapshot, StoreStatus, inspect_at};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::commands::doctor) enum RemediationStatus {
    Created,
    AlreadyWritable,
    Skipped,
    Failed,
}

impl RemediationStatus {
    pub(in crate::commands::doctor) fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::AlreadyWritable => "already_writable",
            Self::Skipped => "skipped",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Serialize)]
pub(in crate::commands::doctor) struct RemediationOutcome {
    pub(in crate::commands::doctor) status: RemediationStatus,
    pub(in crate::commands::doctor) message: String,
    #[serde(skip)]
    pub(in crate::commands::doctor) error: Option<StorageError>,
}

pub(in crate::commands::doctor) struct RemediationRun {
    pub(in crate::commands::doctor) outcome: RemediationOutcome,
    pub(in crate::commands::doctor) store: StoreSnapshot,
}

pub(in crate::commands::doctor) fn initialize_missing(initial: StoreSnapshot) -> RemediationRun {
    let directory = initial.directory.clone();
    if matches!(&initial.status, StoreStatus::Missing) {
        return match create_missing_directory(&directory) {
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
                let store = inspect_at(directory.clone());
                let outcome = match &store.status {
                    StoreStatus::Writable => RemediationOutcome {
                        status: RemediationStatus::AlreadyWritable,
                        message: format!(
                            "Store directory '{}' became writable before doctor created it; no change was needed.",
                            directory.display()
                        ),
                        error: None,
                    },
                    status => RemediationOutcome {
                        status: RemediationStatus::Skipped,
                        message: format!(
                            "Store path '{}' changed during repair and was not overwritten: {}.",
                            directory.display(),
                            status.description()
                        ),
                        error: None,
                    },
                };
                RemediationRun { outcome, store }
            }
            Err(source) => {
                let error = StorageError::CreateStore {
                    path: directory.clone(),
                    source,
                };
                RemediationRun {
                    outcome: failed(error.to_string(), Some(error)),
                    store: inspect_at(directory),
                }
            }
            Ok(()) => {
                let store = inspect_at(directory.clone());
                let outcome = match &store.status {
                    StoreStatus::Writable => RemediationOutcome {
                        status: RemediationStatus::Created,
                        message: format!(
                            "Created and verified missing store directory '{}'.",
                            directory.display()
                        ),
                        error: None,
                    },
                    status => {
                        let description = status.description();
                        failed(
                            format!(
                                concat!(
                                    "Store directory '{}' was created but could not be verified ",
                                    "writable: {}."
                                ),
                                directory.display(),
                                description
                            ),
                            None,
                        )
                    }
                };
                RemediationRun { outcome, store }
            }
        };
    }

    if matches!(&initial.status, StoreStatus::Writable) {
        return RemediationRun {
            outcome: RemediationOutcome {
                status: RemediationStatus::AlreadyWritable,
                message: format!(
                    "Store directory '{}' is already writable; no repair was needed.",
                    directory.display()
                ),
                error: None,
            },
            store: initial,
        };
    }

    let description = initial.status.description();
    RemediationRun {
        outcome: RemediationOutcome {
            status: RemediationStatus::Skipped,
            message: format!(
                concat!(
                    "Did not change existing store path '{}': {}. ",
                    "Inspect its owner and permissions manually."
                ),
                directory.display(),
                description
            ),
            error: None,
        },
        store: initial,
    }
}

#[cfg(test)]
pub(in crate::commands::doctor::storage) fn initialize_missing_at(
    directory: PathBuf,
) -> RemediationRun {
    initialize_missing(inspect_at(directory))
}

fn create_missing_directory(directory: &Path) -> io::Result<()> {
    if let Some(parent) = directory
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    fs::create_dir(directory)
}

fn failed(message: String, error: Option<StorageError>) -> RemediationOutcome {
    RemediationOutcome {
        status: RemediationStatus::Failed,
        message,
        error,
    }
}
