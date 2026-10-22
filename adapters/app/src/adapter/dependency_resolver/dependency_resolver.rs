//! Native Flutter and SwiftPM dependency resolution.
//! Resolve dependency native Flutter và SwiftPM.

use super::swift_registry_tag::swift_registry_tag;
use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use async_trait::async_trait;
use mgc_lib_adapter::native::engine::{resolve_with_protocol, resolve_with_protocol_including_dev};
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::PubProtocol;
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{Manifest, MgResult, ResolvedGraph};

#[async_trait]
impl DependencyResolver for AppAdapter {
    /// Evidence: Flutter resolves through the native pub.dev engine (Phase
    /// 2); Swift resolves through the native SwiftPM engine (Phase 2);
    /// other app languages stay toolchain-owned (gradle/pod) and fail
    /// closed.
    /// Dẫn chứng: Flutter resolve qua engine pub.dev native (Phase 2);
    /// Swift resolve qua engine SwiftPM native (Phase 2); ngôn ngữ app
    /// khác vẫn do toolchain giữ (gradle/pod) và fail-closed.
    fn probe_dependency_resolver(&self) -> MgResult<()> {
        // React Native's internal tier resolver is not exposed as a usable
        // dependency operation until the complete install can commit all
        // tiers atomically. Keep the public capability fail-closed.
        // (Chưa expose resolve RN khi install ba tier chưa atomic.)
        match self.language {
            AppLanguage::Flutter | AppLanguage::Swift => Ok(()),
            AppLanguage::ReactNative => Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "resolve",
                "React Native dependency resolution is not exposed until JS, Android and iOS can be installed and committed atomically",
            )),
            AppLanguage::Kotlin
            | AppLanguage::ObjC
            | AppLanguage::Multi
            | AppLanguage::Maui
            | AppLanguage::Tauri => Err(mgc_types::MgError::Unsupported {
                core: "app",
                capability: "resolve",
                guidance: format!(
                    "{} has no native resolve engine; dependency resolution is owned by its toolchain",
                    self.language.as_str()
                ),
            }),
        }
    }

    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.pending_install.clear()?;
        match self.language {
            AppLanguage::Flutter => {
                let protocol = PubProtocol::from_env();
                let expanded = crate::manifest::flutter::expand_flutter_sdk_dependencies(
                    &self.project_root,
                    manifest,
                )?;
                let resolution = resolve_with_protocol_including_dev(
                    &protocol,
                    EcosystemTag::Dart,
                    "pub://pub.dev",
                    &expanded,
                )
                .await?;
                self.pending_install.store_lock(resolution.lock_packages)?;
                Ok(resolution.graph)
            }
            // Swift registry archives are handled natively; Git-source
            // dependencies fail closed until MGC owns Git transport.
            // (Archive Swift registry được xử lý native; dependency Git bị
            // từ chối tới khi MGC tự sở hữu transport.)
            AppLanguage::Swift => {
                let protocol = mgc_resolver::protocols::SwiftRegistryProtocol::from_env();
                let registry = swift_registry_tag(&protocol);
                let resolution =
                    resolve_with_protocol(&protocol, EcosystemTag::Swift, &registry, manifest)
                        .await?;
                self.pending_install
                    .store_swift(resolution.lock_packages, protocol)?;
                Ok(resolution.graph)
            }
            // RN has internal per-tier experiments, but none is exposed as
            // a supported resolve operation until install/lock commit is
            // atomic across ecosystems. Kotlin/ObjC/Multi remain unsupported.
            // (RN chưa expose resolve khi chưa atomic; Kotlin/ObjC/Multi unsupported.)
            AppLanguage::ReactNative
            | AppLanguage::Kotlin
            | AppLanguage::ObjC
            | AppLanguage::Multi
            | AppLanguage::Maui
            | AppLanguage::Tauri => Err(mgc_types::MgError::Unsupported {
                core: "app",
                capability: "resolve",
                guidance: format!(
                    "{} does not yet expose a complete MagiCore-owned dependency lifecycle; \
                         install and lock publication must be implemented before resolve is advertised",
                    self.language.as_str(),
                ),
            }),
        }
    }
}
