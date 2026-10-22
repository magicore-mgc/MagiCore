//! Pair execution results with guarded-environment and Windows-handle cleanup.
//! Ghép kết quả thực thi với cleanup môi trường và Windows handle.

use super::super::error::SpawnError;
use crate::run::implementation::{EnvironmentError, ExecReport};
use anyhow::Result;

pub(in crate::run::implementation::spawn::implementation) fn combine_execution_and_handle_cleanup(
    execution: Result<ExecReport>,
    cleanup: Option<anyhow::Error>,
) -> Result<ExecReport> {
    match (execution, cleanup) {
        (execution, None) => execution,
        (Ok(_), Some(cleanup)) => Err(cleanup),
        (Err(execution), Some(cleanup)) => {
            Err(SpawnError::ExecutionAndWindowsHandleCleanup { execution, cleanup }.into())
        }
    }
}

pub(in crate::run::implementation::spawn::implementation) fn combine_execution_and_environment_cleanup(
    execution: Result<ExecReport>,
    cleanup: Result<(), EnvironmentError>,
) -> Result<ExecReport> {
    match (execution, cleanup) {
        (Ok(report), Ok(())) => Ok(report),
        (Err(execution), Ok(())) => Err(execution),
        (Ok(_), Err(cleanup)) => Err(cleanup.into()),
        (Err(execution), Err(cleanup)) => {
            Err(SpawnError::ExecutionAndEnvironmentCleanup { execution, cleanup }.into())
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
