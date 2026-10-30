//! Typed errors for Windows Job root-exit cleanup.
//! Lỗi có kiểu cho cleanup khi root Windows Job thoát.

use std::{fmt, process::ExitStatus};

#[derive(Debug)]
pub(in crate::run::implementation) enum RootExitError {
    DescendantInspection(anyhow::Error),
    ActiveMemberEnumeration(anyhow::Error),
    Termination {
        phase: &'static str,
        source: anyhow::Error,
    },
    OutputDrain(anyhow::Error),
    ForbiddenChild(anyhow::Error),
    UninspectedChildren {
        process_ids: String,
        status: String,
    },
    SurvivingChildren {
        process_ids: String,
    },
}

impl fmt::Display for RootExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DescendantInspection(_) => {
                formatter.write_str("failed to inspect Windows Job descendants after root exit")
            }
            Self::ActiveMemberEnumeration(_) => formatter
                .write_str("failed to enumerate active Windows Job members after root exit"),
            Self::Termination { phase, .. } => write!(
                formatter,
                "failed to terminate Windows Job descendants during {phase}"
            ),
            Self::OutputDrain(_) => {
                formatter.write_str("failed to drain child output after Windows Job root exit")
            }
            Self::ForbiddenChild(source) => source.fmt(formatter),
            Self::UninspectedChildren {
                process_ids,
                status,
            } => write!(
                formatter,
                "command root exited while uninspected child process(es) {process_ids} appeared in its Windows job; the process tree was terminated (status: {status})"
            ),
            Self::SurvivingChildren { process_ids } => write!(
                formatter,
                "command root exited while child process(es) {process_ids} remained in its Windows job; the process tree was terminated"
            ),
        }
    }
}

impl std::error::Error for RootExitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DescendantInspection(source)
            | Self::ActiveMemberEnumeration(source)
            | Self::OutputDrain(source)
            | Self::ForbiddenChild(source) => Some(source.as_ref()),
            Self::Termination { source, .. } => Some(source.as_ref()),
            Self::UninspectedChildren { .. } | Self::SurvivingChildren { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;

#[allow(dead_code)]
fn _exit_status_is_rendered_without_platform_assumptions(status: ExitStatus) -> String {
    status.to_string()
}
