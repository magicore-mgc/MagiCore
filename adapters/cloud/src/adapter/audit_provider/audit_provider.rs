//! Preserves Web delegation, Terraform provider-lock scanning and shared-manifest audit planning. — Giữ ủy quyền Web, quét Terraform provider-lock và lập kế hoạch audit manifest dùng chung.

use async_trait::async_trait;
use mgc_types::PackageAdapter;
use std::path::Path;

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use mgc_types::capabilities::AuditProvider;

use super::super::CloudAdapter;

#[async_trait]
impl AuditProvider for CloudAdapter {
    /// Evidence: terraform provider-lock lane + polyglot engine + web
    /// delegate (below). Dẫn chứng: lane terraform provider-lock + engine
    /// polyglot + web delegate (bên dưới).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        if let Some(web) = &self.web {
            return web.audit(project_root).await;
        }
        // P2 2026-09-10: cloud now owns its TARGET-ecosystem lane —
        // Terraform provider-lock provenance — while sibling app
        // manifests still join through the polyglot engine.
        // Cloud giờ có lane ecosystem ĐÍCH — nguồn gốc provider-lock
        // Terraform — manifest app kề vẫn qua engine polyglot.
        let mut plan = mgc_audit::plan_for_shared_manifests(project_root)?;
        let has_terraform = project_root.join(".terraform.lock.hcl").is_file();
        if has_terraform {
            let root = project_root.to_path_buf();
            plan.add_step(mgc_audit::ScanStep {
                ecosystem: "cloud/terraform",
                scanner: "terraform-provider-lock",
                run: Box::new(move || {
                    let root = root.clone();
                    Box::pin(async move { mgc_audit::scanners::audit_terraform_lock(&root) })
                }),
            });
        }
        let manifest = self.parse_manifest(project_root).await?;
        let label = format!(
            "cloud ({} dependencies not scanned — no scanner implemented yet)",
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
