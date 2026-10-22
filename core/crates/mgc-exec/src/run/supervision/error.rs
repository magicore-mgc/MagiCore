//! Typed failures from child-process waiting and output collection.
//! Lỗi typed từ vòng chờ process con và thu thập output.

use crate::run::implementation::output::StreamReadError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub(super) enum SupervisionError {
    OutputRead {
        stream: &'static str,
        source: StreamReadError,
    },
    OutputReaderDisconnected {
        stream: &'static str,
    },
    WaitChild(io::Error),
    KillChild {
        action: &'static str,
        source: io::Error,
    },
    #[cfg(any(unix, windows))]
    TerminationFailure {
        action: &'static str,
        trigger: anyhow::Error,
        termination: Option<anyhow::Error>,
        child_kill: Option<anyhow::Error>,
    },
    ProcessInspection {
        phase: &'static str,
        status: std::process::ExitStatus,
        source: anyhow::Error,
    },
    #[cfg(unix)]
    ProcessGroupDescendants {
        status: std::process::ExitStatus,
    },
}

impl fmt::Display for SupervisionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputRead { stream, source } => {
                write!(formatter, "failed to collect child {stream}: {source}")
            }
            Self::OutputReaderDisconnected { stream } => {
                write!(
                    formatter,
                    "child {stream} reader exited without returning a result"
                )
            }
            Self::WaitChild(source) => {
                write!(formatter, "failed to wait for child process: {source}")
            }
            Self::KillChild { action, source } => {
                write!(
                    formatter,
                    "failed to stop child process while {action}: {source}"
                )
            }
            #[cfg(any(unix, windows))]
            Self::TerminationFailure {
                action,
                trigger,
                termination,
                child_kill,
            } => {
                write!(
                    formatter,
                    "failed to terminate child process tree while {action}"
                )?;
                if let Some(source) = termination {
                    write!(formatter, "; process-tree termination failed: {source}")?;
                }
                if let Some(source) = child_kill {
                    write!(formatter, "; direct child kill failed: {source}")?;
                }
                write!(formatter, "; original failure: {trigger}")
            }
            Self::ProcessInspection {
                phase,
                status,
                source,
            } => write!(
                formatter,
                "cannot verify child process tree {phase}; command was terminated fail-closed (status: {status}): {source}"
            ),
            #[cfg(unix)]
            Self::ProcessGroupDescendants { status } => write!(
                formatter,
                "command root exited while child processes remained in its process group; the process tree was terminated (status: {status})"
            ),
        }
    }
}

impl std::error::Error for SupervisionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OutputRead { source, .. } => Some(source),
            Self::WaitChild(source) => Some(source),
            Self::KillChild { source, .. } => Some(source),
            #[cfg(any(unix, windows))]
            Self::TerminationFailure {
                termination,
                child_kill,
                trigger,
                ..
            } => termination
                .as_deref()
                .or(child_kill.as_deref())
                .or(Some(trigger.as_ref()))
                .map(|source| source as &(dyn std::error::Error + 'static)),
            Self::ProcessInspection { source, .. } => Some(source.as_ref()),
            #[cfg(unix)]
            Self::OutputReaderDisconnected { .. } | Self::ProcessGroupDescendants { .. } => None,
            #[cfg(not(unix))]
            Self::OutputReaderDisconnected { .. } => None,
        }
    }
}
