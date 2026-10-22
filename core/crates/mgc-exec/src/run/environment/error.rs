//! Typed failures from creating and preparing the guarded environment.
//! Lỗi typed khi tạo và chuẩn bị môi trường thực thi được bảo vệ.

use std::path::PathBuf;
use std::time::SystemTimeError;
use std::{fmt, io};

#[derive(Debug)]
pub(in crate::run) enum EnvironmentError {
    AllocationAttemptsExceeded {
        attempts: usize,
    },
    SystemClock(SystemTimeError),
    CreateDirectory(io::Error),
    CreateBlocker {
        tool: String,
        source: io::Error,
    },
    SealDirectory(io::Error),
    RestoreDirectoryPermissions(io::Error),
    RemoveDirectory {
        path: PathBuf,
        source: io::Error,
    },
    CleanupAfterSetup {
        setup: Box<Self>,
        cleanup: Box<Self>,
    },
    JoinPaths(std::env::JoinPathsError),
    #[cfg(windows)]
    ResolveCurrentExecutable(io::Error),
    #[cfg(windows)]
    ExecutableHashCacheUnavailable,
    #[cfg(windows)]
    OpenExecutable {
        path: PathBuf,
        source: io::Error,
    },
    #[cfg(windows)]
    ReadExecutable {
        path: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationAttemptsExceeded { attempts } => write!(
                formatter,
                "could not allocate a unique package-manager deny directory after {attempts} attempts"
            ),
            Self::CreateDirectory(source) => {
                write!(
                    formatter,
                    "failed to create package-manager deny directory: {source}"
                )
            }
            Self::SystemClock(source) => {
                write!(formatter, "system clock is before the Unix epoch: {source}")
            }
            Self::CreateBlocker { tool, source } => {
                write!(formatter, "failed to create the {tool} deny shim: {source}")
            }
            Self::SealDirectory(source) => {
                write!(
                    formatter,
                    "failed to seal package-manager deny directory: {source}"
                )
            }
            Self::RestoreDirectoryPermissions(source) => write!(
                formatter,
                "failed to restore package-manager deny directory permissions: {source}"
            ),
            Self::RemoveDirectory { path, source } => write!(
                formatter,
                "failed to remove package-manager deny directory '{}': {source}",
                path.display()
            ),
            Self::CleanupAfterSetup { setup, cleanup } => write!(
                formatter,
                "environment setup failed ({setup}); cleanup also failed ({cleanup})"
            ),
            Self::JoinPaths(source) => {
                write!(formatter, "failed to compose guarded PATH: {source}")
            }
            #[cfg(windows)]
            Self::ResolveCurrentExecutable(source) => {
                write!(formatter, "failed to resolve current executable: {source}")
            }
            #[cfg(windows)]
            Self::ExecutableHashCacheUnavailable => formatter
                .write_str("executable identity cache became unavailable after initialization"),
            #[cfg(windows)]
            Self::OpenExecutable { path, source } => write!(
                formatter,
                "failed to open executable for identity check '{}': {source}",
                path.display()
            ),
            #[cfg(windows)]
            Self::ReadExecutable { path, source } => write!(
                formatter,
                "failed to read executable for identity check '{}': {source}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for EnvironmentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CreateDirectory(source) => Some(source),
            Self::SystemClock(source) => Some(source),
            Self::CreateBlocker { source, .. }
            | Self::SealDirectory(source)
            | Self::RestoreDirectoryPermissions(source) => Some(source),
            Self::RemoveDirectory { source, .. } => Some(source),
            Self::JoinPaths(source) => Some(source),
            Self::CleanupAfterSetup { setup, .. } => Some(setup.as_ref()),
            Self::AllocationAttemptsExceeded { .. } => None,
            #[cfg(windows)]
            Self::ResolveCurrentExecutable(source)
            | Self::OpenExecutable { source, .. }
            | Self::ReadExecutable { source, .. } => Some(source),
            #[cfg(windows)]
            Self::ExecutableHashCacheUnavailable => None,
        }
    }
}
