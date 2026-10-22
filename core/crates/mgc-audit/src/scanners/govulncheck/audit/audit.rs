//! Dispatch Go audit to the native OSV fallback when go.mod exists.
//! (Điều phối audit Go sang OSV fallback khi có go.mod.)

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

pub async fn audit_go(project_root: &Path) -> MgResult<AuditReport> {
    if !project_root.join("go.mod").is_file() {
        return Ok(AuditReport::scanner_failed(
            "mgc-go-osv",
            "no go.mod found — the Go scanner only audits Go modules",
        ));
    }
    super::osv_fallback::audit_go_osv_fallback(project_root).await
}
