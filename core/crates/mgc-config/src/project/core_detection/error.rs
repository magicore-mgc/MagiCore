//! Core-detection errors and recovery guidance.
//! Lỗi nhận diện core và hướng xử lý.
use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

/// Next-step guidance; classification never performs the action.
/// Hướng xử lý tiếp theo; phân loại không tự thực hiện hành động.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(super) enum RecoveryAction {
    InspectManifest,
    RepairProjectIdentity,
    SelectCore,
}

/// Errors owned by project-core detection.
/// Lỗi thuộc owner nhận diện core project.
#[derive(Debug)]
pub(super) enum CoreDetectionError {
    Source(anyhow::Error),
    ConflictingManifestDeclarations {
        cores: Vec<String>,
        marker_file: String,
    },
    UnknownDeclaredCore {
        core: String,
        marker_file: String,
    },
    AmbiguousProjectCore {
        project_root: PathBuf,
        signatures: Vec<String>,
        marker_file: String,
    },
}

pub(super) type Result<T> = std::result::Result<T, CoreDetectionError>;

impl CoreDetectionError {
    /// Classify recovery without retrying, rewriting or guessing automatically.
    /// Phân loại khắc phục nhưng không tự retry, ghi lại hay đoán core.
    #[cfg(test)]
    pub(super) fn recovery_action(&self) -> RecoveryAction {
        match self {
            Self::Source(_) => RecoveryAction::InspectManifest,
            Self::ConflictingManifestDeclarations { .. } | Self::UnknownDeclaredCore { .. } => {
                RecoveryAction::RepairProjectIdentity
            }
            Self::AmbiguousProjectCore { .. } => RecoveryAction::SelectCore,
        }
    }

    /// Convert at the existing public boundary while preserving anyhow causes.
    /// Chuyển lỗi tại biên public hiện tại và giữ root cause của anyhow.
    pub(super) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Source(error) => error,
            domain_error => anyhow::Error::msg(domain_error.to_string()),
        }
    }
}

impl fmt::Display for CoreDetectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => write!(formatter, "{error}"),
            Self::ConflictingManifestDeclarations { cores, marker_file } => write!(
                formatter,
                "conflicting MagiCore core declarations in project manifests: {}. Set one authoritative '{}' marker or mgc.toml ecosystem.",
                cores.join(", "),
                marker_file,
            ),
            Self::UnknownDeclaredCore { core, marker_file } => write!(
                formatter,
                "manifest declares unknown MagiCore core '{}'; use a supported core in '{}' or mgc.toml",
                core, marker_file,
            ),
            Self::AmbiguousProjectCore {
                project_root,
                signatures,
                marker_file,
            } => write!(
                formatter,
                "Ambiguous project core in '{}': multiple ecosystem manifests ({}) but no '{}'. Run 'mgc init --signature <core>' to write a plain-text core marker explicitly.",
                project_root.display(),
                signatures.join(", "),
                marker_file,
            ),
        }
    }
}

impl StdError for CoreDetectionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Source(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for CoreDetectionError {
    fn from(error: anyhow::Error) -> Self {
        Self::Source(error)
    }
}
