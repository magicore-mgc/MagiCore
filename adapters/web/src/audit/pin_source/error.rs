//! Own typed pin-source failures and preserve the existing MgError mapping.
//! Sở hữu lỗi typed của pin source và giữ nguyên cách ánh xạ MgError.

use mgc_types::MgError;

#[derive(Debug)]
pub(in crate::audit) enum PinSourceError {
    Lockfile(MgError),
    RivalLockfiles(Vec<String>),
}

impl From<MgError> for PinSourceError {
    fn from(error: MgError) -> Self {
        Self::Lockfile(error)
    }
}

impl From<PinSourceError> for MgError {
    fn from(error: PinSourceError) -> Self {
        match error {
            PinSourceError::Lockfile(error) => error,
            PinSourceError::RivalLockfiles(names) => MgError::Other(rival_lockfile_message(&names)),
        }
    }
}

impl std::fmt::Display for PinSourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lockfile(error) => write!(formatter, "{error}"),
            Self::RivalLockfiles(names) => {
                write!(formatter, "{}", rival_lockfile_message(names))
            }
        }
    }
}

impl std::error::Error for PinSourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lockfile(error) => Some(error),
            Self::RivalLockfiles(_) => None,
        }
    }
}

fn rival_lockfile_message(names: &[String]) -> String {
    let import_target = if names.iter().any(|name| name.contains("deno")) {
        "deno"
    } else {
        "bun"
    };
    format!(
        "rival lockfile(s) detected [{}] but mgc.lock is missing — mgc audit consumes mgc.lock ONLY. \
         Run `mgc import {import_target}` to migrate this project first.",
        names.join(", ")
    )
}
