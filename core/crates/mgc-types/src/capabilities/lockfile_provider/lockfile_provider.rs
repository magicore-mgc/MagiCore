//! Lockfile provider capability implementation.
//! Triển khai capability lockfile provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;
use crate::manifest::Manifest;
use async_trait::async_trait;
use std::path::Path;

/// Writes the core's manifest/lockfile surface (moved from `PackageAdapter`).
/// Ghi manifest/lockfile của core — di chuyển từ `PackageAdapter`.
#[async_trait]
pub trait LockfileProvider: CoreIdent {
    fn probe_lockfile_provider(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "write_manifest",
            "this core does not use mgc-written manifests; regenerate via `mgc create-<core>` or edit the manifest directly",
        ))
    }

    async fn write_manifest(&self, _project_root: &Path, _manifest: &Manifest) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "write_manifest",
            "this core does not use mgc-written manifests; regenerate via `mgc create-<core>` or edit the manifest directly",
        ))
    }
}
