//! Typed refusals for minimum-release-age install policy.
//! Lỗi từ chối có kiểu cho policy minimum-release-age.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(super) enum AgeGateError {
    UnsupportedResolver { core: String },
    FrozenLockMissingPublicationTimestamps,
}

impl Display for AgeGateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedResolver { core } => write!(
                formatter,
                "minimum-release-age is configured, but core '{core}' has no verified timestamp-aware resolver; refusing to install without the gate"
            ),
            Self::FrozenLockMissingPublicationTimestamps => write!(
                formatter,
                "--frozen cannot verify minimum-release-age because mgc.lock does not store registry publication timestamps; run install without --frozen to resolve again and refresh the lock"
            ),
        }
    }
}

impl std::error::Error for AgeGateError {}
