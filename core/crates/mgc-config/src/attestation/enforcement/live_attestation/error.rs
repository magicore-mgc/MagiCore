//! Typed live-attestation failures / Lỗi typed cho attestation hiện hữu.
//! Keeps policy failures local and preserves their underlying source errors.

use std::fmt;

#[derive(Debug)]
pub(super) enum LiveAttestationError {
    GlobalIdentitiesUnavailable {
        source: anyhow::Error,
    },
    Missing {
        project: String,
        core: String,
    },
    Mismatch {
        project: String,
        core: String,
        reason: String,
    },
    CannotVerify {
        source: anyhow::Error,
    },
}

impl fmt::Display for LiveAttestationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GlobalIdentitiesUnavailable { source } => write!(
                formatter,
                "cannot access global identities dir ({source}) — core attestation cannot be checked"
            ),
            Self::Missing { project, core } => write!(
                formatter,
                concat!(
                    "no core attestation for '{project}' (core '{core}') — run `mgc trust anchor` to ",
                    "attest this project (or set MGC_TRUST_ANCHOR=warn to downgrade)"
                ),
                project = project,
                core = core
            ),
            Self::Mismatch {
                project,
                core,
                reason,
            } => write!(
                formatter,
                concat!(
                    "core attestation mismatch for '{project}' (core '{core}'): {reason} — run ",
                    "`mgc trust anchor --re-attest` after intentional changes"
                ),
                project = project,
                core = core,
                reason = reason
            ),
            Self::CannotVerify { source } => {
                write!(formatter, "cannot verify core attestation ({source})")
            }
        }
    }
}

impl std::error::Error for LiveAttestationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GlobalIdentitiesUnavailable { source } | Self::CannotVerify { source } => {
                Some(source.as_ref())
            }
            Self::Missing { .. } | Self::Mismatch { .. } => None,
        }
    }
}

impl LiveAttestationError {
    pub(super) fn into_anyhow(self) -> anyhow::Error {
        anyhow::Error::new(self)
    }
}
