//! Core-marker domain errors and recovery classification.
//! Lỗi domain của core marker và phân loại cách khắc phục.
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

/// Recovery guidance for a core-marker failure; this type never retries by itself.
/// Hướng xử lý lỗi marker; type này không tự thực hiện retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(super) enum RecoveryAction {
    RetryOperation,
    RepairProjectIdentity,
    InspectSource,
}

/// Typed errors local to the project core-marker owner.
/// Lỗi typed chỉ thuộc owner marker core của project.
#[derive(Debug)]
pub(super) enum CoreMarkerError {
    Source(anyhow::Error),
    ConfiguredCoreReassignment {
        existing: String,
        desired: String,
    },
    InvalidMarker {
        path: PathBuf,
        marker: String,
        known: String,
    },
    MissingConfigEcosystem,
    UnknownConfiguredCore {
        configured: String,
        marker: String,
    },
    MarkerConfigConflict {
        marker: String,
        configured: String,
    },
    UnknownCore {
        core: String,
        known: String,
    },
    ExistingMarkerConflict {
        existing: String,
        desired: String,
    },
    ConcurrentMarkerConflict {
        existing: String,
        desired: String,
    },
    MarkerChangedDuringClaim {
        path: PathBuf,
    },
}

pub(super) type Result<T> = std::result::Result<T, CoreMarkerError>;

impl CoreMarkerError {
    /// Classify next action without silently retrying or repairing project state.
    /// Phân loại bước kế tiếp nhưng không tự retry hay sửa identity project.
    #[cfg(test)]
    pub(super) fn recovery_action(&self) -> RecoveryAction {
        match self {
            Self::Source(_) => RecoveryAction::InspectSource,
            Self::MarkerChangedDuringClaim { .. } => RecoveryAction::RetryOperation,
            _ => RecoveryAction::RepairProjectIdentity,
        }
    }

    /// Convert at the public boundary while retaining the previous anyhow cause.
    /// Chuyển ở biên public nhưng giữ root cause anyhow như trước.
    pub(super) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Source(error) => error,
            domain_error => anyhow::Error::msg(domain_error.to_string()),
        }
    }
}

impl fmt::Display for CoreMarkerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => write!(formatter, "{error}"),
            Self::ConfiguredCoreReassignment { existing, desired } => write!(
                formatter,
                "Project is already configured as core '{existing}'; refusing to replace it with '{desired}'. Core reassignment is not supported by this version."
            ),
            Self::InvalidMarker {
                path,
                marker,
                known,
            } => write!(
                formatter,
                "'{}' has an invalid core marker '{}'. Expected one of: {}. Repair the marker file manually before running core-aware commands.",
                path.display(),
                marker,
                known
            ),
            Self::MissingConfigEcosystem => {
                write!(formatter, "project config is missing its ecosystem")
            }
            Self::UnknownConfiguredCore { configured, marker } => write!(
                formatter,
                "project config has unknown core '{}'; refusing to trust core marker '{}'",
                configured, marker
            ),
            Self::MarkerConfigConflict { marker, configured } => write!(
                formatter,
                "core marker '{}' conflicts with mgc.toml core '{}'; core reassignment is not supported by this version",
                marker, configured
            ),
            Self::UnknownCore { core, known } => {
                write!(
                    formatter,
                    "Unknown core '{}'. Expected one of: {}.",
                    core, known
                )
            }
            Self::ExistingMarkerConflict { existing, desired } => write!(
                formatter,
                "Project is already marked as core '{}'; refusing to replace it with '{}'. Core reassignment is not supported by this version.",
                existing, desired
            ),
            Self::ConcurrentMarkerConflict { existing, desired } => write!(
                formatter,
                "Project is already marked as core '{}'; refusing to replace it with '{}'.",
                existing, desired
            ),
            Self::MarkerChangedDuringClaim { path } => write!(
                formatter,
                "Core marker '{}' changed while claiming project ownership; retry the command.",
                path.display()
            ),
        }
    }
}

impl StdError for CoreMarkerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Source(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for CoreMarkerError {
    fn from(error: anyhow::Error) -> Self {
        Self::Source(error)
    }
}

impl From<io::Error> for CoreMarkerError {
    fn from(error: io::Error) -> Self {
        Self::Source(error.into())
    }
}
