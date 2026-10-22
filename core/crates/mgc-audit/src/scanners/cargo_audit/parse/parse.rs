//! Coordinate cargo-audit JSON parsing without changing its public contract.
//! Điều phối parse JSON cargo-audit mà không đổi contract public.

use super::super::types::CargoAuditParse;
use super::consistency::validate_vulnerabilities;
use super::document::deserialize_report;
use super::error::CargoAuditParseError;
use super::finding::parse_finding;
use mgc_types::MgResult;

/// Parse cargo-audit JSON output (real schema, fail-closed).
/// Parse JSON output của cargo-audit (schema thật, fail-closed).
///
/// Format: https://github.com/rustsec/rustsec/blob/main/cargo-audit/README.md#json-output
pub fn parse_cargo_audit_json(json: &str) -> MgResult<CargoAuditParse> {
    parse_cargo_audit_json_inner(json).map_err(CargoAuditParseError::into_mg_error)
}

fn parse_cargo_audit_json_inner(json: &str) -> Result<CargoAuditParse, CargoAuditParseError> {
    let report = deserialize_report(json)?;
    let packages_audited = report
        .lockfile
        .and_then(|lockfile| lockfile.dependency_count)
        .unwrap_or(0);

    let mut vulnerabilities = Vec::new();
    for finding in &report.vulnerabilities.list {
        vulnerabilities.push(parse_finding(finding)?);
    }
    validate_vulnerabilities(&report.vulnerabilities)?;

    Ok(CargoAuditParse {
        vulnerabilities,
        packages_audited,
    })
}
