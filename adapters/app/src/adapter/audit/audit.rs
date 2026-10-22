//! App audit dispatch.
//! Điều phối audit App.

use crate::adapter::AppAdapter;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use mgc_types::capabilities::AuditProvider;
use std::path::Path;

#[async_trait]
impl AuditProvider for AppAdapter {
    /// Evidence: real scanner dispatch per language
    /// (crate::audit::run_audit — Kotlin→OWASP; Flutter/Swift honest
    /// unavailable). Dẫn chứng: điều phối scanner thật theo ngôn ngữ
    /// (crate::audit::run_audit).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Real scanner dispatch per language — Kotlin→OWASP dependency-check;
        // Flutter/Swift honestly unavailable (no CVE scanner exists).
        // Điều phối scanner thật theo ngôn ngữ — Kotlin→dependency-check;
        // Flutter/Swift trung thực unavailable (không có scanner CVE).
        crate::audit::run_audit(self.language, project_root).await
    }
}
