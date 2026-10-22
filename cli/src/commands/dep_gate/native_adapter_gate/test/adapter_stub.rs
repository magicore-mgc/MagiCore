//! Minimal PackageAdapter test double — adapter giả tối thiểu cho test.
//! Supplies only identity, capability and update evidence used by the gate.
//! Chỉ cung cấp identity, capability và evidence update cho gate.

use async_trait::async_trait;
use mgc_types::adapter::{InstalledPackage, ManifestIdentity, PackageAdapter};
use mgc_types::capabilities::{
    ArtifactFetcher, AuditProvider, ContentStoreProvider, CoreIdent, DependencyResolver,
    DeployProvider, DeviceProvider, LifecycleRunner, LockfileProvider, Materializer,
    ModelRuntimeProvider, OptimizerProvider, ProjectDetector, ScaffoldProvider, SimulatorProvider,
};
use mgc_types::ecosystem::Ecosystem;
use mgc_types::error::{MgError, MgResult};
use mgc_types::manifest::Manifest;
use std::path::Path;

pub(super) struct AdapterStub {
    pub(super) identity: Option<ManifestIdentity>,
    pub(super) capabilities: &'static [mgc_types::capabilities::Capability],
    pub(super) native_update: bool,
}

impl CoreIdent for AdapterStub {
    fn core_id(&self) -> &'static str {
        "ai"
    }

    fn name(&self) -> &str {
        "test adapter"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Ai
    }
}

impl ProjectDetector for AdapterStub {
    fn can_handle(&self, _project_root: &Path) -> bool {
        false
    }
}

impl DependencyResolver for AdapterStub {}
impl ArtifactFetcher for AdapterStub {}
impl ContentStoreProvider for AdapterStub {}
impl LockfileProvider for AdapterStub {}
impl AuditProvider for AdapterStub {}
impl ScaffoldProvider for AdapterStub {}
impl LifecycleRunner for AdapterStub {}
impl OptimizerProvider for AdapterStub {}
impl Materializer for AdapterStub {}
impl SimulatorProvider for AdapterStub {}
impl DeviceProvider for AdapterStub {}
impl DeployProvider for AdapterStub {}
impl ModelRuntimeProvider for AdapterStub {}

#[async_trait]
impl PackageAdapter for AdapterStub {
    async fn parse_manifest(&self, _project_root: &Path) -> MgResult<Manifest> {
        Err(MgError::Other("unused adapter stub".to_string()))
    }

    async fn list(&self, _project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        Ok(Vec::new())
    }

    fn capabilities(&self) -> &'static [mgc_types::capabilities::Capability] {
        self.capabilities
    }

    fn manifest_identity(&self) -> Option<ManifestIdentity> {
        self.identity.clone()
    }

    fn supports_native_update(&self) -> bool {
        self.native_update
    }
}

pub(super) fn ai_identity() -> ManifestIdentity {
    ManifestIdentity {
        core: "ai".to_string(),
        language: "python".to_string(),
        format: "pyproject.toml".to_string(),
        relpath: "pyproject.toml".to_string(),
    }
}
