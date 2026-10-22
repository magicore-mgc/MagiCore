//! Typed missing-marker failures / Lỗi typed cho marker thiếu.
//! Keeps missing-marker policy failures local with source preservation.

use std::fmt;

#[derive(Debug)]
pub(super) enum MissingMarkerAttestationError {
    GlobalIdentitiesUnavailable { source: anyhow::Error },
    MissingIdentity { project: String, marker: String },
    Mismatch { project: String, reason: String },
    CannotVerify { source: anyhow::Error },
}

impl fmt::Display for MissingMarkerAttestationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GlobalIdentitiesUnavailable { source } => write!(
                formatter,
                "cannot access global identities dir ({source}) — missing core marker cannot be checked"
            ),
            Self::MissingIdentity { project, marker } => write!(
                formatter,
                concat!(
                    "project core marker and attestation are missing for '{project}' — restore ",
                    "'{marker}' or run `mgc init --signature <core>` and ",
                    "`mgc trust anchor`"
                ),
                project = project,
                marker = marker
            ),
            Self::Mismatch { project, reason } => write!(
                formatter,
                concat!(
                    "core identity attestation mismatch for '{project}': {reason} — restore the ",
                    "trusted project identity before continuing"
                ),
                project = project,
                reason = reason
            ),
            Self::CannotVerify { source } => {
                write!(
                    formatter,
                    "cannot verify core identity attestation ({source})"
                )
            }
        }
    }
}

impl std::error::Error for MissingMarkerAttestationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GlobalIdentitiesUnavailable { source } | Self::CannotVerify { source } => {
                Some(source.as_ref())
            }
            Self::MissingIdentity { .. } | Self::Mismatch { .. } => None,
        }
    }
}

impl MissingMarkerAttestationError {
    pub(super) fn into_anyhow(self) -> anyhow::Error {
        anyhow::Error::new(self)
    }
}
