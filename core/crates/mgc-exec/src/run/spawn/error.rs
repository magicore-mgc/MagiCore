//! Typed failures raised while preparing or starting a child process.
//! Lỗi typed khi chuẩn bị hoặc khởi chạy process con.

use crate::run::implementation::EnvironmentError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub(super) enum SpawnError {
    CancelledBeforeSpawn,
    CompatRunnerRequiresAuditLog,
    ResolveCurrentDirectory(io::Error),
    #[cfg(windows)]
    CanonicalizeWorkingDirectory(io::Error),
    CommandSpawnFailed {
        command: String,
        source: io::Error,
    },
    ExecutionAndEnvironmentCleanup {
        execution: anyhow::Error,
        cleanup: EnvironmentError,
    },
    #[cfg(windows)]
    PendingWindowsHandleCleanup(anyhow::Error),
    ExecutionAndWindowsHandleCleanup {
        execution: anyhow::Error,
        cleanup: anyhow::Error,
    },
    #[cfg(windows)]
    WindowsProcessTreeSetupCleanup {
        activation: anyhow::Error,
        termination: Option<anyhow::Error>,
        kill: Option<io::Error>,
        wait: Option<io::Error>,
    },
}

impl fmt::Display for SpawnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CancelledBeforeSpawn => {
                formatter.write_str("execution cancelled by its supervisor before spawn")
            }
            Self::CompatRunnerRequiresAuditLog => {
                formatter.write_str("--compat-runner requires an audit log path; refusing to spawn")
            }
            Self::ResolveCurrentDirectory(source) => {
                write!(
                    formatter,
                    "cannot resolve current execution directory: {source}"
                )
            }
            #[cfg(windows)]
            Self::CanonicalizeWorkingDirectory(source) => {
                write!(
                    formatter,
                    "cannot canonicalize execution directory: {source}"
                )
            }
            Self::CommandSpawnFailed { command, source } => {
                write!(formatter, "failed to spawn '{command}': {source}")
            }
            Self::ExecutionAndEnvironmentCleanup { execution, cleanup } => write!(
                formatter,
                "execution failed ({execution}); guarded environment cleanup also failed ({cleanup})"
            ),
            #[cfg(windows)]
            Self::PendingWindowsHandleCleanup(cleanup) => write!(
                formatter,
                "a previous Windows handle cleanup failed; refusing new execution: {cleanup}"
            ),
            Self::ExecutionAndWindowsHandleCleanup { execution, cleanup } => write!(
                formatter,
                "execution failed ({execution}); Windows handle cleanup also failed ({cleanup})"
            ),
            #[cfg(windows)]
            Self::WindowsProcessTreeSetupCleanup {
                activation,
                termination,
                kill,
                wait,
            } => {
                write!(
                    formatter,
                    "failed to activate Windows process job: {activation}"
                )?;
                if let Some(error) = termination {
                    write!(formatter, "; job termination also failed: {error}")?;
                }
                if let Some(error) = kill {
                    write!(formatter, "; child kill also failed: {error}")?;
                }
                if let Some(error) = wait {
                    write!(formatter, "; child wait also failed: {error}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SpawnError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ResolveCurrentDirectory(source) | Self::CommandSpawnFailed { source, .. } => {
                Some(source)
            }
            #[cfg(windows)]
            Self::CanonicalizeWorkingDirectory(source) => Some(source),
            Self::ExecutionAndEnvironmentCleanup { execution, .. } => Some(execution.as_ref()),
            #[cfg(windows)]
            Self::PendingWindowsHandleCleanup(cleanup) => Some(cleanup.as_ref()),
            Self::ExecutionAndWindowsHandleCleanup { execution, .. } => Some(execution.as_ref()),
            #[cfg(windows)]
            Self::WindowsProcessTreeSetupCleanup { activation, .. } => Some(activation.as_ref()),
            Self::CancelledBeforeSpawn | Self::CompatRunnerRequiresAuditLog => None,
        }
    }
}
