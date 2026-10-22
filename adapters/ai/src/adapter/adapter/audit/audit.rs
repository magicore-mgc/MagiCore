//! Aggregates shared dependency scanners and the AI model scanner.
//! Gom scanner dependency dùng chung và scanner model AI.

use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, PackageAdapter};
use mgc_types::capabilities::AuditProvider;
use std::path::Path;

use super::super::AiAdapter;
use super::super::model_artifact::model_artifact_report;

#[async_trait]
impl AuditProvider for AiAdapter {
    /// Evidence: the two-layer aggregate below (dependency scanners +
    /// model-artifact scanner). Dẫn chứng: aggregate hai lớp bên dưới
    /// (scanner dependency + scanner model-artifact).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Dependency lanes (R1): shared constructors — the same
        // detection rule every core uses. The python rule is the broad
        // shared one (requirements variants, pylock, uv.lock,
        // pyproject): recognized-but-unscannable manifests surface as
        // the scanner's honest Failed, never a hidden skip.
        // Lane dependency: constructor chung — cùng luật mọi core.
        let mut plan = mgc_audit::AuditPlan::new();
        for step in [
            mgc_audit::python_step(project_root),
            mgc_audit::rust_step(project_root),
            mgc_audit::go_step(project_root),
        ]
        .into_iter()
        .flatten()
        {
            plan.add_step(step);
        }

        // Model artifact layer: scan the AI model directory conventions.
        // Lớp model artifact: scan theo thư mục model quen thuộc của AI.
        for model_dir in ["models", "model", "artifacts", "checkpoints"] {
            let dir = project_root.join(model_dir);
            if dir.is_dir() {
                let dir = dir.clone();
                plan.add_step(mgc_audit::ScanStep {
                    ecosystem: "model-artifact",
                    scanner: "mgc-model-scanner",
                    run: Box::new(move || {
                        let dir = dir.clone();
                        Box::pin(async move { model_artifact_report(&dir) })
                    }),
                });
            }
        }

        if plan.is_empty() {
            let manifest = self.parse_manifest(project_root).await?;
            return Ok(AuditReport::unsupported_ecosystem(format!(
                "ai ({} dependencies not scanned — no scanner implemented yet)",
                manifest.all_dependencies().count()
            )));
        }

        plan.execute().await
    }
}
