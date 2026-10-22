//! Package.resolved discovery, bounded input, audit status and skipped-pin aggregation.
//! Tìm lockfile, đọc dữ liệu giới hạn và điều phối audit Swift.

use super::super::{read_swift_resolved, swift_resolved_path};
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Swift/SPM audit: Package.resolved → OSV `swift` ecosystem queries.
/// Located via the shared bounded predicate (same scope the adapter
/// gate sees — REVIEW F1).
pub async fn audit_swift_spam(project_root: &Path) -> MgResult<AuditReport> {
    let resolved_path = swift_resolved_path(project_root);
    let Some(path) = resolved_path else {
        return Ok(AuditReport::unsupported_ecosystem(
            "swift/swiftpm (no Package.resolved found — run swift package resolve first)",
        ));
    };
    let raw = crate::scanners::read_bounded_project_input(&path, "Package.resolved")?;
    let (pins, skipped) = read_swift_resolved(&raw)?;
    super::super::report_swift_pins(pins, skipped).await
}
