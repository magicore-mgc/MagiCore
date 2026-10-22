//! Typed failures observed while scanning and signaling a Unix process tree.
//! Lỗi typed phát hiện khi quét và gửi signal cây process Unix.

use std::{fmt, io};

/// Bound retained termination diagnostics when a process tree repeatedly fails to stop.
/// Giới hạn lỗi được giữ lại khi cây process liên tục không dừng được.
const MAX_RETAINED_TERMINATION_FAILURES: usize = 8;

#[derive(Debug, Clone, Copy)]
pub(super) enum TerminationPhase {
    TermSweep,
    KillSweep,
}

#[derive(Debug)]
enum TerminationIssue {
    ProcessScan {
        phase: TerminationPhase,
        source: anyhow::Error,
    },
    ProcessGroupSignal {
        group: u32,
        signal: libc::c_int,
        source: io::Error,
    },
    ProcessGroupProbe {
        group: u32,
        source: io::Error,
    },
    ProcessGroupIdentity {
        pid: u32,
        source: io::Error,
    },
    RootSignal {
        pid: u32,
        signal: libc::c_int,
        source: io::Error,
    },
}

#[derive(Debug)]
pub(in crate::run::implementation) struct TerminationError {
    failures: Vec<TerminationIssue>,
    omitted_failures: usize,
}

impl TerminationError {
    pub(in crate::run::implementation) fn process_group_probe(
        group: u32,
        source: io::Error,
    ) -> Self {
        Self {
            failures: vec![TerminationIssue::ProcessGroupProbe { group, source }],
            omitted_failures: 0,
        }
    }

    pub(in crate::run::implementation) fn process_group_identity(
        pid: u32,
        source: io::Error,
    ) -> Self {
        Self {
            failures: vec![TerminationIssue::ProcessGroupIdentity { pid, source }],
            omitted_failures: 0,
        }
    }
}

#[derive(Default)]
pub(super) struct TerminationFailures {
    failures: Vec<TerminationIssue>,
    total_failures: usize,
}

impl TerminationFailures {
    pub(super) fn process_scan(&mut self, phase: TerminationPhase, source: anyhow::Error) {
        self.record(TerminationIssue::ProcessScan { phase, source });
    }

    pub(super) fn process_group_signal(
        &mut self,
        group: u32,
        signal: libc::c_int,
        source: io::Error,
    ) {
        self.record(TerminationIssue::ProcessGroupSignal {
            group,
            signal,
            source,
        });
    }

    pub(super) fn process_group_probe(&mut self, group: u32, source: io::Error) {
        self.record(TerminationIssue::ProcessGroupProbe { group, source });
    }

    pub(super) fn root_signal(&mut self, pid: u32, signal: libc::c_int, source: io::Error) {
        self.record(TerminationIssue::RootSignal {
            pid,
            signal,
            source,
        });
    }

    pub(super) fn into_result(self) -> std::result::Result<(), TerminationError> {
        if self.total_failures == 0 {
            return Ok(());
        }
        let omitted_failures = self.total_failures.saturating_sub(self.failures.len());
        Err(TerminationError {
            failures: self.failures,
            omitted_failures,
        })
    }

    fn record(&mut self, issue: TerminationIssue) {
        self.total_failures = self.total_failures.saturating_add(1);
        if self.failures.len() < MAX_RETAINED_TERMINATION_FAILURES {
            self.failures.push(issue);
        }
    }
}

impl fmt::Display for TerminationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, failure) in self.failures.iter().enumerate() {
            if index > 0 {
                formatter.write_str("; additional termination failure: ")?;
            }
            write_issue(formatter, failure)?;
        }
        if self.omitted_failures > 0 {
            write!(
                formatter,
                "; {} additional termination failure(s) omitted after the diagnostic limit",
                self.omitted_failures
            )?;
        }
        Ok(())
    }
}

fn write_issue(formatter: &mut fmt::Formatter<'_>, issue: &TerminationIssue) -> fmt::Result {
    match issue {
        TerminationIssue::ProcessScan { phase, source } => {
            write!(formatter, "process scan failed during {phase:?}: {source}")
        }
        TerminationIssue::ProcessGroupSignal {
            group,
            signal,
            source,
        } => write!(
            formatter,
            "failed to send {} to process group {group}: {source}",
            signal_name(*signal)
        ),
        TerminationIssue::ProcessGroupProbe { group, source } => {
            write!(
                formatter,
                "failed to inspect process group {group}: {source}"
            )
        }
        TerminationIssue::ProcessGroupIdentity { pid, source } => {
            write!(
                formatter,
                "failed to read process group for PID {pid}: {source}"
            )
        }
        TerminationIssue::RootSignal {
            pid,
            signal,
            source,
        } => write!(
            formatter,
            "failed to send {} to child process {pid}: {source}",
            signal_name(*signal)
        ),
    }
}

impl std::error::Error for TerminationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.failures.first()? {
            TerminationIssue::ProcessScan { source, .. } => Some(source.as_ref()),
            TerminationIssue::ProcessGroupSignal { source, .. }
            | TerminationIssue::ProcessGroupProbe { source, .. }
            | TerminationIssue::ProcessGroupIdentity { source, .. }
            | TerminationIssue::RootSignal { source, .. } => Some(source),
        }
    }
}

fn signal_name(signal: libc::c_int) -> &'static str {
    match signal {
        libc::SIGTERM => "SIGTERM",
        libc::SIGKILL => "SIGKILL",
        _ => "unknown signal",
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
