//! Typed refusal reasons for Windows npm probe authentication.
//! Lý do từ chối typed khi xác thực npm probe trên Windows.

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::run::implementation::windows_job::inspection) enum NpmProbeAuthenticationError {
    #[cfg(windows)]
    ImageNameMismatch,
    #[cfg(windows)]
    UnsafeCommandShape,
    #[cfg(windows)]
    ExecutableDigestUnavailable,
    ImagePathUnavailable,
    #[cfg(windows)]
    ImageDigestUnavailable,
    ImageDigestMismatch,
    UnsupportedPlatform,
}

impl fmt::Display for NpmProbeAuthenticationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            #[cfg(windows)]
            Self::ImageNameMismatch => "process image name is not npm.exe",
            #[cfg(windows)]
            Self::UnsafeCommandShape => "argv is not the exact npm version probe",
            #[cfg(windows)]
            Self::ExecutableDigestUnavailable => "running MGC executable hash is unavailable",
            Self::ImagePathUnavailable => "process image path is unavailable",
            #[cfg(windows)]
            Self::ImageDigestUnavailable => "process image hash is unavailable",
            Self::ImageDigestMismatch => {
                "process image hash does not match the running MGC executable"
            }
            Self::UnsupportedPlatform => {
                "Windows image authentication is unavailable on this platform"
            }
        })
    }
}

impl std::error::Error for NpmProbeAuthenticationError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
