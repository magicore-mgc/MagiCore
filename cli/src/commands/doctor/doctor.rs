//! Coordinates the doctor report and its CLI output.
//! Điều phối report doctor và output CLI.

use anyhow::Result;

use super::{
    analysis, command::DoctorCmd, model::DoctorReport, presentation, registry, storage, toolchain,
};

pub fn report() -> Result<DoctorReport> {
    let toolchain = toolchain_inventory()?;
    let store = storage::inspect();
    let registries = registry::statuses();

    Ok(analysis::build(toolchain, store, registries))
}

fn report_with_fix() -> Result<(DoctorReport, storage::RemediationOutcome)> {
    let toolchain = toolchain_inventory()?;
    let store = storage::inspect();
    let registries = registry::statuses();
    let run = storage::initialize_missing(store);

    Ok((
        analysis::build(toolchain, run.store, registries),
        run.outcome,
    ))
}

/// Probe the general tools plus the current project's core tools when detected.
/// Probe tool chung cộng tool của core hiện tại khi nhận diện được.
///
/// Detection is read-only: it reads `.mgc.core`, `mgc.toml` or manifests and
/// never writes a marker (unlike the identity-claiming resolution path).
/// Nhận diện chỉ-đọc: đọc `.mgc.core`, `mgc.toml` hoặc manifest, không ghi
/// marker (khác đường resolve có claim identity).
fn toolchain_inventory() -> Result<Vec<super::model::Tool>> {
    let cwd = std::env::current_dir()?;
    match mgc_config::project::ProjectConfig::detect_core(&cwd) {
        // A split fullstack project's server half runs on its own toolchain, so
        // readiness must cover that runner too, not just the core's row.
        // (Nửa server của project fullstack tách rời chạy bằng toolchain riêng, nên
        // readiness phải bao phủ runner đó, không chỉ hàng của core.)
        Ok(Some(core)) => Ok(toolchain::inventory_for_project(&core, &cwd)),
        // No core is not a doctor error: the general inventory still runs.
        // Không có core không phải lỗi doctor: inventory chung vẫn chạy.
        Ok(None) => Ok(toolchain::inventory()),
        Err(_) => Ok(toolchain::inventory()),
    }
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
