//! `lockfile_provider.rs` — Lockfile-provider bridge / cầu nối provider lockfile.
use super::super::WebAdapter;
use crate::manifest::write_manifest;
use async_trait::async_trait;
use mgc_types::{Manifest, MgResult, capabilities::LockfileProvider};
use std::path::Path;

#[async_trait]
impl LockfileProvider for WebAdapter {
    fn probe_lockfile_provider(&self) -> MgResult<()> {
        Ok(())
    }

    /// Evidence: real package.json writer (manifest::write_manifest) and
    /// the mgc.lock lane (lockfile.rs).
    /// Dẫn chứng: bộ viết package.json thật (manifest::write_manifest) và
    /// lane mgc.lock (lockfile.rs).
    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        write_manifest(project_root, manifest)
    }
}
