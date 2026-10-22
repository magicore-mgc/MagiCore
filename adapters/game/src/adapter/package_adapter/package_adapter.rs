use super::super::GameAdapter;
use crate::GameEngine;
use async_trait::async_trait;
use mgc_types::adapter::{InstalledPackage, PackageAdapter};
use mgc_types::capabilities::Capability;
use mgc_types::{Ecosystem, Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl PackageAdapter for GameAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        Self::CAPABILITIES
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        // Source-verified manifest per engine (engine.rs detection).
        // (Manifest theo engine — đúng file detect đọc.)
        let (language, format, relpath) = match self.engine {
            GameEngine::Bevy => ("bevy", "Cargo.toml", "Cargo.toml"),
            GameEngine::Godot => ("godot", "project.godot", "project.godot"),
            GameEngine::Unity => ("unity", "manifest.json", "Packages/manifest.json"),
            GameEngine::Unreal => ("unreal", "*.uproject", "*.uproject"),
        };
        Some(mgc_types::ManifestIdentity {
            core: "game".to_string(),
            language: language.to_string(),
            format: format.to_string(),
            relpath: relpath.to_string(),
        })
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        match self.engine {
            GameEngine::Bevy => {
                mgc_adapter_base::cargo_manifest::parse_manifest(project_root, Ecosystem::Game)
            }
            GameEngine::Godot | GameEngine::Unity | GameEngine::Unreal => {
                let name = project_root
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "game".to_string());
                Ok(Manifest::new(&name, Ecosystem::Game))
            }
        }
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        let _ = project_root;
        Err(mgc_types::MgError::Unsupported {
            core: "game",
            capability: "list",
            guidance: format!(
                "{} dependency declarations are not verified installed-state; the game adapter has no installed package inventory",
                self.engine.as_str()
            ),
        })
    }
}
