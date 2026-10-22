use super::super::GameAdapter;
use crate::GameEngine;
use async_trait::async_trait;
use mgc_types::capabilities::LockfileProvider;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl LockfileProvider for GameAdapter {
    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        match self.engine {
            GameEngine::Bevy => {
                mgc_adapter_base::cargo_manifest::write_manifest(project_root, manifest)
            }
            GameEngine::Godot | GameEngine::Unity | GameEngine::Unreal => {
                // Fail-closed (hardware precedent): the old silent Ok no-op
                // faked a manifest write that never happened.
                // Fail-closed (tiền lệ hardware): Ok no-op cũ giả một lần
                // ghi manifest không bao giờ xảy ra.
                Err(mgc_types::MgError::Unsupported {
                    core: "game",
                    capability: "write_manifest",
                    guidance: format!(
                        "{} projects do not have a MagiCore-owned dependency manifest; dependency mutation is unsupported",
                        self.engine.as_str()
                    ),
                })
            }
        }
    }
}
