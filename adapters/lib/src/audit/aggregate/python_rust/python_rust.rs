//! Aggregate Python and Rust scanners in the shared audit engine.
//! Tổng hợp scanner Python và Rust bằng engine audit dùng chung.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Aggregate python + rust scans through the shared engine.
/// Tổng hợp scan python + rust qua engine chung.
pub(in crate::audit) async fn aggregate_python_rust(project_root: &Path) -> MgResult<AuditReport> {
    // Steps from the shared constructors (R1) — identical scanners,
    // one detection rule.
    // Step từ constructor chung — cùng scanner, một luật nhận diện.
    let mut plan = mgc_audit::AuditPlan::new();
    for step in [
        mgc_audit::python_step(project_root),
        mgc_audit::rust_step(project_root),
    ]
    .into_iter()
    .flatten()
    {
        plan.add_step(step);
    }

    plan.execute().await
}
