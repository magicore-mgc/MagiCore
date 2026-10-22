//! Coordinates the doctor report and its CLI output.
//! Điều phối report doctor và output CLI.

use anyhow::Result;

use super::{
    analysis, command::DoctorCmd, model::DoctorReport, presentation, registry, storage, toolchain,
};

pub fn report() -> Result<DoctorReport> {
    let toolchain = toolchain::inventory();
    let store = storage::inspect();
    let registries = registry::statuses();

    Ok(analysis::build(toolchain, store, registries))
}

fn report_with_fix() -> Result<(DoctorReport, storage::RemediationOutcome)> {
    let toolchain = toolchain::inventory();
    let store = storage::inspect();
    let registries = registry::statuses();
    let run = storage::initialize_missing(store);

    Ok((
        analysis::build(toolchain, run.store, registries),
        run.outcome,
    ))
}

pub fn handle(cmd: DoctorCmd) -> Result<()> {
    let DoctorCmd::Check { json, fix } = cmd;
    let (report, remediation) = if fix {
        let (report, remediation) = report_with_fix()?;
        (report, Some(remediation))
    } else {
        (report()?, None)
    };

    if json {
        println!(
            "{}",
            presentation::render_json(&report, remediation.as_ref())?
        );
    } else {
        print!(
            "{}",
            presentation::render_human(&report, remediation.as_ref())
        );
    }

    if let Some(outcome) = remediation {
        ensure_remediation_completed(outcome)?;
    }

    Ok(())
}

fn ensure_remediation_completed(outcome: storage::RemediationOutcome) -> Result<()> {
    match outcome.status {
        storage::RemediationStatus::Created | storage::RemediationStatus::AlreadyWritable => Ok(()),
        storage::RemediationStatus::Skipped | storage::RemediationStatus::Failed => {
            if let Some(error) = outcome.error {
                Err(error.into())
            } else {
                Err(anyhow::anyhow!(outcome.message))
            }
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
