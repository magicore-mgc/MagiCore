//! Test-only adapter fixture for capability owners.
//! Fixture adapter chỉ dùng trong test cho các owner capability.

use crate::capabilities::{
    ArtifactFetcher, AuditProvider, ContentStoreProvider, CoreIdent, DependencyResolver,
    DeployProvider, DeviceProvider, LifecycleRunner, LockfileProvider, Materializer,
    ModelRuntimeProvider, OptimizerProvider, ProjectDetector, ScaffoldProvider, SimulatorProvider,
};
use crate::ecosystem::Ecosystem;
use crate::error::MgError;
use std::path::Path;

pub(in crate::capabilities) struct MockCore;

impl CoreIdent for MockCore {
    fn core_id(&self) -> &'static str {
        "web"
    }
    fn name(&self) -> &str {
        "Web"
    }
    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Web
    }
}

impl ProjectDetector for MockCore {
    fn can_handle(&self, _project_root: &Path) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl DependencyResolver for MockCore {}
#[async_trait::async_trait]
impl ArtifactFetcher for MockCore {}
#[async_trait::async_trait]
impl ContentStoreProvider for MockCore {}
#[async_trait::async_trait]
impl LockfileProvider for MockCore {}
#[async_trait::async_trait]
impl AuditProvider for MockCore {}
impl ScaffoldProvider for MockCore {}
impl LifecycleRunner for MockCore {}
impl OptimizerProvider for MockCore {}
impl Materializer for MockCore {}
impl SimulatorProvider for MockCore {}
impl DeviceProvider for MockCore {}
impl DeployProvider for MockCore {}
impl ModelRuntimeProvider for MockCore {}

pub(in crate::capabilities) fn assert_unsupported(
    error: MgError,
    expected_core: &'static str,
    expected_capability: &'static str,
    expected_guidance: &str,
) {
    match error {
        MgError::Unsupported {
            core,
            capability,
            guidance,
        } => {
            assert_eq!(core, expected_core);
            assert_eq!(capability, expected_capability);
            assert_eq!(guidance, expected_guidance);
            assert_eq!(
                MgError::Unsupported {
                    core,
                    capability,
                    guidance
                }
                .to_string(),
                format!(
                    "{expected_core} core does not support '{expected_capability}' yet: {expected_guidance}"
                ),
            );
        }
        other => panic!("expected Unsupported error, got {other:?}"),
    }
}
