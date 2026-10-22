//! Swift Package.resolved audit entry point.
//! Entry point audit Swift Package.resolved.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

pub async fn audit_swift(project_root: &Path) -> MgResult<AuditReport> {
    // P2 2026-09-10: Package.resolved pins → OSV.dev `swift` ecosystem
    // — a REAL CVE scan via the shared OSV client.
    // Package.resolved ghim → OSV.dev ecosystem `swift` — scan CVE THẬT
    // qua OSV client dùng chung.
    mgc_audit::scanners::audit_swift_spam(project_root).await
}
