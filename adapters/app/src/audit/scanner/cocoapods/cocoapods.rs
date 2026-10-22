//! CocoaPods lockfile audit entry point.
//! Entry point audit lockfile CocoaPods.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

pub async fn audit_cocoapods(project_root: &Path) -> MgResult<AuditReport> {
    // P2 2026-09-10: Podfile.lock pins → OSV.dev `pods` ecosystem.
    // Podfile.lock ghim → OSV.dev ecosystem `pods`.
    mgc_audit::scanners::audit_cocoapods_osv(project_root).await
}
