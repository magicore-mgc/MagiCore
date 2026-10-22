//! Owns the AI adapter lock/manifest capability forwarding, including native-project ownership revalidation.
//! Sở hữu chuyển tiếp capability lock/manifest của adapter AI, gồm xác thực lại ownership project native.

use crate::adapter::AiAdapter;
use async_trait::async_trait;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl mgc_types::capabilities::LockfileProvider for AiAdapter {
    fn probe_lockfile_provider(&self) -> MgResult<()> {
        match &self.python_lane {
            Some(py) => py.probe_lockfile_provider(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "write_manifest",
                "no pyproject.toml — no mgc-written manifest surface for this ai project",
            )),
        }
    }

    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        match &self.python_lane {
            // S09b/D012: manifest write goes straight to the shared Python
            // service (same body the lib facade calls); the ownership guard
            // replicates `LibAdapter::require_python_native_at` byte-for-byte
            // so the fail-closed surface is unchanged.
            // (Ghi manifest thẳng qua service dùng chung; guard giữ nguyên.)
            Some(_) => {
                if !mgc_python_service::manifest::supports_native_python_project(project_root) {
                    return Err(mgc_types::capabilities::unsupported_capability(
                        "lib",
                        "write_manifest",
                        "this Python project uses dependency sources or lockfiles not owned by the MGC native PEP 621 lane; migrate explicitly or use a separately selected compatibility manager",
                    ));
                }
                mgc_python_service::manifest::write_pyproject_manifest(project_root, manifest)
            }
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "write_manifest",
                "no pyproject.toml — no mgc-written manifest surface for this ai project",
            )),
        }
    }
}
