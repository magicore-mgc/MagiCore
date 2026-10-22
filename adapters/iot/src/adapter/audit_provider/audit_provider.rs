//! Keeps the shared polyglot audit route and the existing unsupported scanner evidence for unscanned dependencies. — Giữ route audit polyglot dùng chung và evidence scanner unsupported hiện có cho dependency chưa quét.

use mgc_types::PackageAdapter;
use std::path::Path;

use super::super::IotAdapter;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use mgc_types::capabilities::AuditProvider;
#[async_trait]
impl AuditProvider for IotAdapter {
    /// Evidence: shared polyglot dispatch below (Rust/Python/Go manifests
    /// get real scans; pure-embedded stays honestly unsupported).
    /// Dẫn chứng: dispatch polyglot bên dưới (manifest Rust/Python/Go
    /// được quét thật; thuần embedded giữ unsupported trung thực).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Shared-engine polyglot dispatch (P2): Rust/Python/Go manifests
        // in IoT projects get real scans; pure-embedded manifests stay
        // honestly unsupported.
        // Dispatch polyglot qua engine chung: manifest Rust/Python/Go
        // trong project IoT được quét thật; manifest thuần embedded giữ
        // trung thực unsupported.
        let manifest = self.parse_manifest(project_root).await?;
        mgc_audit::audit_polyglot(
            project_root,
            format!(
                "iot ({} dependencies not scanned — no scanner implemented yet)",
                manifest.all_dependencies().count()
            ),
        )
        .await
    }
}
