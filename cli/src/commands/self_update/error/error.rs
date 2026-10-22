//! Typed errors for the self-update lifecycle.
//! Lỗi có kiểu cho từng ranh giới trong vòng đời self-update.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum SelfUpdateError {
    InvalidVersion(String),
    UnsupportedVariant(String),
    UnsupportedHost(String),
    ChecksumMismatch {
        archive: String,
        expected: String,
        actual: String,
    },
    NoBinary(String),
    DownloadFailed {
        url: String,
        detail: String,
    },
    ProbeFailed(String),
    ManifestInvalid(String),
    SignatureInvalid(String),
    UnsignedRefused(String),
}
impl SelfUpdateError {
    fn wrap(error: Self) -> anyhow::Error {
        anyhow::Error::new(error)
    }
    pub(crate) fn invalid_version(x: &str) -> anyhow::Error {
        Self::wrap(Self::InvalidVersion(x.to_owned()))
    }
    pub(crate) fn unsupported_variant(x: &str) -> anyhow::Error {
        Self::wrap(Self::UnsupportedVariant(x.to_owned()))
    }
    pub(crate) fn unsupported_host(x: &str) -> anyhow::Error {
        Self::wrap(Self::UnsupportedHost(x.to_owned()))
    }
    pub(crate) fn checksum_mismatch(a: &str, e: &str, x: &str) -> anyhow::Error {
        Self::wrap(Self::ChecksumMismatch {
            archive: a.to_owned(),
            expected: e.to_owned(),
            actual: x.to_owned(),
        })
    }
    pub(crate) fn no_binary(x: &str) -> anyhow::Error {
        Self::wrap(Self::NoBinary(x.to_owned()))
    }
    pub(crate) fn download_failed(u: &str, d: &str) -> anyhow::Error {
        Self::wrap(Self::DownloadFailed {
            url: u.to_owned(),
            detail: d.to_owned(),
        })
    }
    pub(crate) fn probe_failed(x: &str) -> anyhow::Error {
        Self::wrap(Self::ProbeFailed(x.to_owned()))
    }
    pub(crate) fn manifest_invalid(x: &str) -> anyhow::Error {
        Self::wrap(Self::ManifestInvalid(x.to_owned()))
    }
    pub(crate) fn signature_invalid(x: &str) -> anyhow::Error {
        Self::wrap(Self::SignatureInvalid(x.to_owned()))
    }
    pub(crate) fn unsigned_refused(x: &str) -> anyhow::Error {
        Self::wrap(Self::UnsignedRefused(x.to_owned()))
    }
}
impl Display for SelfUpdateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVersion(x) => write!(
                f,
                "invalid version: {x} (expected like 1.1.0-rc.10, with or without a leading 'v')"
            ),
            Self::UnsupportedVariant(x) => {
                write!(f, "unsupported variant: {x} (magicore|magicore-web)")
            }
            Self::UnsupportedHost(x) => write!(
                f,
                "unsupported {x} for self-update (linux/macos/windows on x64/arm64 only)"
            ),
            Self::ChecksumMismatch {
                archive,
                expected,
                actual,
            } => write!(
                f,
                "checksum mismatch for {archive}: expected {expected}, got {actual}"
            ),
            Self::NoBinary(x) => write!(f, "archive {x} contains no mgc binary"),
            Self::DownloadFailed { url, detail } => write!(f, "download {url} failed: {detail}"),
            Self::ProbeFailed(x) => write!(
                f,
                "new binary failed verification ({x}) — previous version restored"
            ),
            Self::ManifestInvalid(x) => write!(f, "release manifest rejected: {x}"),
            Self::SignatureInvalid(x) => write!(f, "release manifest signature invalid: {x}"),
            Self::UnsignedRefused(x) => write!(f, "unsigned self-update refused: {x}"),
        }
    }
}
impl std::error::Error for SelfUpdateError {}
