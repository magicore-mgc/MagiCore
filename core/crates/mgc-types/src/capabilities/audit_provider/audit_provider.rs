//! Audit provider capability implementation.
//! Triển khai capability audit provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::adapter::AuditReport;
use crate::error::MgResult;
use async_trait::async_trait;
use std::path::Path;

/// Runs the security scanners for the core's ecosystems.
/// Chạy scanner bảo mật cho các ecosystem của core.
#[async_trait]
pub trait AuditProvider: CoreIdent {
    fn probe_audit_provider(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "audit",
            "no scanner implemented for this core",
        ))
    }

    async fn audit(&self, _project_root: &Path) -> MgResult<AuditReport> {
        Err(unsupported_capability(
            self.core_id(),
            "audit",
            "no scanner implemented for this core",
        ))
    }
}
