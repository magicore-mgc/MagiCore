use super::super::GameAdapter;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::PackageAdapter;
use mgc_types::adapter::AuditReport;
use mgc_types::capabilities::AuditProvider;
use std::path::Path;

#[async_trait]
impl AuditProvider for GameAdapter {
    /// Evidence: shared-engine polyglot dispatch (real cargo-audit for
    /// Bevy Cargo.toml; honest UnsupportedEcosystem label otherwise).
    /// Dẫn chứng: dispatch polyglot qua engine chung (cargo-audit thật
    /// cho Cargo.toml Bevy; nhãn UnsupportedEcosystem trung thực còn lại).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Shared-engine polyglot dispatch (P2 2026-09-09): scan every
        // manifest the shared scanners understand (Bevy games ship
        // Cargo.toml → real cargo-audit); engines without any shared
        // manifest keep the honest UnsupportedEcosystem label.
        // Dispatch polyglot qua engine chung: quét mọi manifest mà
        // scanner chung hiểu (game Bevy có Cargo.toml → cargo-audit
        // thật); engine không còn manifest chung nào thì giữ nhãn
        // UnsupportedEcosystem trung thực.
        let manifest = self.parse_manifest(project_root).await?;
        mgc_audit::audit_polyglot(
            project_root,
            format!(
                "game ({} dependencies not scanned — no scanner implemented yet)",
                manifest.all_dependencies().count()
            ),
        )
        .await
    }
}
