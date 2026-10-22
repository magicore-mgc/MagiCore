//! Owns the PackageAdapter façade methods, manifest identity/ownership and parse/list behavior. PlatformIO parsing is called only through the local parser owner. — Sở hữu method façade PackageAdapter, manifest identity/ownership và hành vi parse/list. Parser PlatformIO chỉ được gọi qua owner cục bộ.

use std::path::Path;

use super::super::IotAdapter;
use super::super::platformio_manifest::parse_platformio_manifest;
use crate::framework::IotFramework;
use async_trait::async_trait;
use mgc_types::adapter::{InstalledPackage, PackageAdapter};
use mgc_types::capabilities::Capability;
use mgc_types::{Ecosystem, Manifest, MgResult};
#[async_trait]
impl PackageAdapter for IotAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        Self::CAPABILITIES
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        // Source-verified manifest per framework (only Esp32Rust is
        // mgc-written; Platformio/Zephyr are toolchain-owned and never
        // reach staging, but their identity still mismatches correctly).
        // (Manifest theo framework.)
        let (language, format, relpath) = match self.framework {
            IotFramework::Esp32Rust => ("esp32-rust", "Cargo.toml", "Cargo.toml"),
            IotFramework::Platformio => ("platformio", "platformio.ini", "platformio.ini"),
            IotFramework::Zephyr => ("zephyr", "west.yml", "west.yml"),
        };
        Some(mgc_types::ManifestIdentity {
            core: "iot".to_string(),
            language: language.to_string(),
            format: format.to_string(),
            relpath: relpath.to_string(),
        })
    }

    fn manifest_owned(&self) -> bool {
        // platformio.ini / west.yml are owned SOLELY by their toolchains
        // (write_manifest fails closed for them — same precedent as
        // game/iot non-manifest engines): booking an add in memory would
        // fake a mutation the tool never sees. Only the esp32-rust
        // Cargo.toml is mgc-written.
        // (platformio.ini/west.yml do toolchain sở hữu DUY NHẤT.)
        matches!(self.framework, IotFramework::Esp32Rust)
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        match self.framework {
            IotFramework::Esp32Rust => {
                mgc_adapter_base::cargo_manifest::parse_manifest(project_root, Ecosystem::Iot)
            }
            IotFramework::Platformio => parse_platformio_manifest(project_root),
            IotFramework::Zephyr => {
                let name = project_root
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "firmware".to_string());
                Ok(Manifest::new(&name, Ecosystem::Iot))
            }
        }
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        let _ = project_root;
        Err(mgc_types::MgError::Unsupported {
            core: "iot",
            capability: "list",
            guidance: format!(
                "{} dependency declarations are not verified installed-state; the IoT adapter has no installed package inventory",
                self.framework.as_str()
            ),
        })
    }
}
