//! Typed failures owned by the Hardware build lane.
//! Lỗi typed thuộc lane build Hardware.

use std::fmt;
use std::path::PathBuf;

/// PlatformIO refusal and missing project detection belong to Hardware build.
#[derive(Debug)]
pub(super) enum HardwareBuildError {
    Unsupported {
        target: &'static str,
        guidance: &'static str,
    },
    FrameworkNotDetected {
        root: PathBuf,
    },
}

impl fmt::Display for HardwareBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { target, guidance } => write!(
                formatter,
                "`mgc build` is not supported for core '{target}': {guidance}"
            ),
            Self::FrameworkNotDetected { root } => {
                write!(
                    formatter,
                    "No hardware framework detected in {}",
                    root.display()
                )
            }
        }
    }
}

impl std::error::Error for HardwareBuildError {}
