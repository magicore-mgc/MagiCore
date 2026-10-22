use crate::adapter::types::LibAdapter;
use async_trait::async_trait;
use mgc_types::AuditProvider;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

#[async_trait]
impl AuditProvider for LibAdapter {
    /// Evidence: Rust uses MGC's native OSV scanner; TypeScript delegates
    /// to the Web aggregate and other languages use shared MGC audit routes.
    /// Dẫn chứng: Rust dùng scanner OSV native của MGC; TypeScript ủy quyền
    /// cho Web aggregate, ngôn ngữ khác dùng route audit MGC dùng chung.
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        if let Some(web) = &self.web {
            return web.audit(project_root).await;
        }
        // MGC-owned scanner dispatch per language; Rust uses native OSV.
        // Điều phối scanner MGC sở hữu theo ngôn ngữ; Rust dùng OSV native.
        crate::audit::run_audit(self.language, project_root).await
    }
}
