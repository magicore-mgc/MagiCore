//! Machine-format rendering (JSON / SARIF / CycloneDX) for audit reports.
//! Render định dạng máy (JSON / SARIF / CycloneDX) cho report audit.

use anyhow::Result;

use crate::commands::audit::OutputFormat;
use mgc_types::adapter::AuditReport;

/// exit contract lives in enforce_audit_exit, shared with the table).
/// Finisher đường máy: chỉ in payload cấu trúc.
/// (Machine-path rendering — print only.)
pub(crate) fn print_machine_payload(report: &AuditReport, fmt: OutputFormat) -> Result<()> {
    match fmt {
        OutputFormat::Json => {
            let envelope = mgc_audit::output::json::JsonAuditEnvelope::from_report(report);
            println!("{}", envelope.to_json()?);
        }
        OutputFormat::Sarif => {
            let log = mgc_audit::output::sarif::to_sarif(report);
            println!("{}", serde_json::to_string_pretty(&log)?);
        }
        OutputFormat::CycloneDx => {
            let bom = mgc_audit::output::cyclonedx::to_cyclonedx(report);
            println!("{}", serde_json::to_string_pretty(&bom)?);
        }
        OutputFormat::Table => unreachable!("table handled by the human path"),
    }
    Ok(())
}
