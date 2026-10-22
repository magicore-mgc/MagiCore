//! Route hardware package audits through the shared polyglot engine.
//! Chuyển audit package hardware qua engine đa ngôn ngữ dùng chung.

use super::super::adapter::HardwareAdapter;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use mgc_types::capabilities::AuditProvider;
use std::path::Path;

#[async_trait]
impl AuditProvider for HardwareAdapter {
    /// Evidence: shared-engine polyglot dispatch below (sibling Rust/Go
    /// manifests get real scans; pure HDL stays honestly unsupported).
    /// Dẫn chứng: dispatch polyglot bên dưới (manifest kề Rust/Go được
    /// quét thật; thuần HDL giữ unsupported trung thực).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Shared-engine polyglot dispatch (P2): template packages carry
        // no dependency graph of their own, but sibling manifests (a
        // Rust tooling crate next to the HDL) get real scans; pure HDL
        // projects stay honestly unsupported.
        // Dispatch polyglot qua engine chung: template package không có
        // dependency graph riêng, nhưng manifest kề bên (crate Rust công
        // cụ cạnh HDL) được quét thật; project thuần HDL giữ trung thực
        // unsupported.
        mgc_audit::audit_polyglot(
            project_root,
            "hardware (template packages only — no scanner implemented yet)".to_string(),
        )
        .await
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
