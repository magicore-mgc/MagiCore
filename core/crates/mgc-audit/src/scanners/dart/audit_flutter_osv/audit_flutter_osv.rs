//! Flutter lockfile audit orchestration.
//! Điều phối audit lockfile Flutter.

use crate::scanners::osv::audit_osv_pins;
use crate::scanners::read_bounded_project_input;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

use super::preflight_report::report_without_registry_query;
use super::skipped_projection::apply_skipped_packages;

pub(in crate::scanners::dart) async fn run(project_root: &Path) -> MgResult<AuditReport> {
    let lock_path = project_root.join("pubspec.lock");
    if !lock_path.is_file() {
        return Ok(AuditReport::unsupported_ecosystem(
            "flutter/dart (no pubspec.lock found — run flutter pub get first)",
        ));
    }

    let raw = read_bounded_project_input(&lock_path, "pubspec.lock")?;
    let (pins, skipped) = super::super::pubspec_lock::read(&raw)?;
    if let Some(report) = report_without_registry_query(&pins, &skipped) {
        return Ok(report);
    }

    let report = audit_osv_pins(&pins).await?;
    let report = apply_skipped_packages(report, pins.len(), &skipped);
    Ok(report)
}

#[cfg(test)]
#[path = "test/audit_flutter_osv.rs"]
mod tests;
