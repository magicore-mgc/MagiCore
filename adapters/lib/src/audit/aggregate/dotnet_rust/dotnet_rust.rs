//! Aggregate .NET and Rust while preserving unsupported .NET results.
//! Tổng hợp .NET và Rust, giữ cả kết quả .NET unsupported.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Aggregate dotnet + rust scans (polyglot lib with packages.lock.json
/// AND Cargo.toml).
/// Tổng hợp scan dotnet + rust (polyglot lib có packages.lock.json
/// VÀ Cargo.toml).
pub(in crate::audit) async fn aggregate_dotnet_rust(project_root: &Path) -> MgResult<AuditReport> {
    // Same language-gated fallback as java (DotNet without a root
    // .csproj): keep the honest Unsupported in the aggregate.
    // Fallback như java: giữ Unsupported trung thực trong aggregate.
    let mut plan = mgc_audit::AuditPlan::new();
    match mgc_audit::dotnet_step(project_root) {
        Some(step) => plan.add_step(step),
        None => {
            let root = project_root.to_path_buf();
            plan.add_step(mgc_audit::ScanStep {
                ecosystem: "dotnet",
                scanner: "osv-nuget",
                run: Box::new(move || {
                    let root = root.clone();
                    Box::pin(async move { mgc_audit::scanners::audit_dotnet(&root).await })
                }),
            });
        }
    }
    if let Some(step) = mgc_audit::rust_step(project_root) {
        plan.add_step(step);
    }

    plan.execute().await
}
