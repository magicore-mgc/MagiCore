//! Run shared-manifest scans and the CI/CD-specific workflow policy scan.
//! Chạy scan manifest dùng chung và scan policy workflow riêng CICD.

use super::super::adapter::CicdAdapter;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, PackageAdapter};
use mgc_types::capabilities::AuditProvider;
use std::path::Path;

#[async_trait]
impl AuditProvider for CicdAdapter {
    /// Evidence: the github-actions policy lane + polyglot engine below.
    /// Dẫn chứng: lane policy github-actions + engine polyglot bên dưới.
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // P2 2026-09-10: cicd owns its TARGET-ecosystem lane — GitHub
        // Actions SHA-pinning/permissions/injection policy — plus the
        // polyglot engine for sibling dependency manifests.
        // Cicd có lane ecosystem ĐÍCH — policy SHA-pinning/permissions/
        // injection của GitHub Actions — cộng engine polyglot cho
        // manifest dependency kề.
        let mut plan = mgc_audit::plan_for_shared_manifests(project_root)?;
        let has_workflows = project_root.join(".github").join("workflows").is_dir();
        if has_workflows {
            let root = project_root.to_path_buf();
            plan.add_step(mgc_audit::ScanStep {
                ecosystem: "cicd/github-actions",
                scanner: "github-actions-policy",
                run: Box::new(move || {
                    let root = root.clone();
                    Box::pin(async move { mgc_audit::scanners::audit_github_actions(&root) })
                }),
            });
        }
        let manifest = self.parse_manifest(project_root).await?;
        let label = format!(
            "cicd ({} dependencies not scanned — no scanner implemented yet)",
            manifest.all_dependencies().count()
        );
        if plan.is_empty() {
            return Ok(mgc_types::adapter::AuditReport::unsupported_ecosystem(
                label,
            ));
        }
        plan.execute().await
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
