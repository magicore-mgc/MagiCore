//! Aggregate Java and Rust while preserving unsupported Java results.
//! Tổng hợp Java và Rust, giữ cả kết quả Java unsupported.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Aggregate java + rust scans (polyglot lib with gradle verification
/// metadata AND Cargo.toml).
/// Tổng hợp scan java + rust (polyglot lib có metadata verification
/// gradle VÀ Cargo.toml).
pub(in crate::audit) async fn aggregate_java_rust(project_root: &Path) -> MgResult<AuditReport> {
    // Language-gated caller (Java without build.gradle is possible —
    // Maven pom.xml projects): when the file-gated constructor declines,
    // still run the lane so its honest Unsupported (with remediation)
    // stays in the aggregate instead of vanishing.
    // Caller theo ngôn ngữ (Java thiếu build.gradle vẫn có — project
    // Maven): constructor từ chối thì vẫn chạy lane để Unsupported
    // trung thực còn trong aggregate.
    let mut plan = mgc_audit::AuditPlan::new();
    match mgc_audit::java_step(project_root) {
        Some(step) => plan.add_step(step),
        None => {
            let root = project_root.to_path_buf();
            plan.add_step(mgc_audit::ScanStep {
                ecosystem: "java",
                scanner: "osv-maven",
                run: Box::new(move || {
                    let root = root.clone();
                    Box::pin(async move { mgc_audit::scanners::audit_java(&root).await })
                }),
            });
        }
    }
    if let Some(step) = mgc_audit::rust_step(project_root) {
        plan.add_step(step);
    }

    plan.execute().await
}
