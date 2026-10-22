//! Typed process-policy errors.
//! Lỗi process-policy có kiểu.

use std::fmt;

#[derive(Debug)]
pub(in crate::run::implementation) enum ProcessPolicyError {
    #[cfg(any(windows, test))]
    MissingChildCommandLine {
        image_name: String,
        process_id: u32,
    },
    ForbiddenScriptInterpreter {
        path: String,
        tool: String,
    },
}

impl fmt::Display for ProcessPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(any(windows, test))]
            Self::MissingChildCommandLine {
                image_name,
                process_id,
            } => write!(
                formatter,
                "cannot read command line for monitored child '{}' PID {process_id}",
                image_name
            ),
            Self::ForbiddenScriptInterpreter { path, tool } => write!(
                formatter,
                "script '{}' uses forbidden package manager '{}' as its interpreter",
                path, tool
            ),
        }
    }
}

impl std::error::Error for ProcessPolicyError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
