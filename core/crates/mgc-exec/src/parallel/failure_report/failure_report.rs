use anyhow::{Result, bail};

use crate::parallel::ParallelResult;
use crate::run::ExecReport;

/// Resolve the recorded first failure or convert all successful reports in input order.
pub(crate) fn resolve_task_results(
    task_results: Vec<(usize, std::result::Result<ExecReport, String>)>,
    failed_index: usize,
) -> Result<Vec<ParallelResult>> {
    if failed_index != usize::MAX {
        let Some((index, result)) = task_results
            .iter()
            .find(|(index, _)| *index == failed_index)
        else {
            bail!("parallel task supervisor lost the first failing task");
        };
        match result {
            Ok(report) => bail!(
                "parallel task {index} failed with exit code {}",
                report.exit_code
            ),
            Err(error) => bail!("parallel task {index} failed: {error}"),
        }
    }

    task_results
        .into_iter()
        .map(|(index, result)| {
            let report = result.map_err(|error| {
                anyhow::anyhow!("parallel task {index} failed unexpectedly: {error}")
            })?;
            Ok(ParallelResult {
                index,
                exit_code: report.exit_code,
                stdout: report.stdout_full,
                stderr: report.stderr_tail,
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
