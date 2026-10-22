//! Aggregate Go and Rust scanners in the shared audit engine.
//! Tổng hợp scanner Go và Rust bằng engine audit dùng chung.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Aggregate go + rust scans through the shared engine (polyglot lib
/// with go.mod AND Cargo.toml — never first-match).
/// Tổng hợp scan go + rust qua engine chung (lib đa ngôn ngữ có go.mod
/// VÀ Cargo.toml — không first-match).
pub(in crate::audit) async fn aggregate_go_rust(project_root: &Path) -> MgResult<AuditReport> {
    let mut plan = mgc_audit::AuditPlan::new();
    for step in [
        mgc_audit::go_step(project_root),
        mgc_audit::rust_step(project_root),
    ]
    .into_iter()
    .flatten()
    {
        plan.add_step(step);
    }

    plan.execute().await
}
