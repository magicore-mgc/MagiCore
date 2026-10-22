//! Typed failures owned by the Game build lane.
//! Lỗi typed thuộc lane build Game.

use std::fmt;
use std::path::PathBuf;

/// Unsupported routes and missing engine detection belong to Game build.
#[derive(Debug)]
pub(super) enum GameBuildError {
    Unsupported {
        target: &'static str,
        guidance: &'static str,
    },
    FrameworkNotDetected {
        root: PathBuf,
    },
}

impl fmt::Display for GameBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { target, guidance } => write!(
                formatter,
                "`mgc build` is not supported for core '{target}': {guidance}"
            ),
            Self::FrameworkNotDetected { root } => {
                write!(
                    formatter,
                    "No game engine framework detected in {}",
                    root.display()
                )
            }
        }
    }
}

impl std::error::Error for GameBuildError {}
