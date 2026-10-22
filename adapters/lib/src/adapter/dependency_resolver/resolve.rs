//! Resolve native Lib dependencies after preserving existing preflight order.
//! Resolve dependency Lib native và giữ nguyên thứ tự preflight.

use crate::adapter::native_registry::{NativeRegistry, NativeRegistryOperation};
use crate::adapter::types::{JavaManifestKind, LibAdapter, ResolveFirst};
use crate::language::LibLanguage;
use mgc_types::DependencyResolver;
use mgc_types::{Manifest, MgResult, ResolvedGraph};

impl LibAdapter {
    /// Resolve through the existing Web delegate or selected native registry lane.
    /// Resolve qua Web delegate hiện tại hoặc native registry lane đã chọn.
    pub(super) async fn resolve_native_dependencies(
        &self,
        manifest: &Manifest,
    ) -> MgResult<ResolvedGraph> {
        self.require_python_native("resolve")?;
        self.require_unique_dotnet_project("resolve")?;
        if let Some(web) = &self.web {
            web.arm_age_gate_for_core(&self.project_root, self.security_core)?;
            return web.resolve(manifest).await;
        }
        self.ensure_age_gate_supported(&self.project_root)?;

        let Some(lane) = resolver_lane(self.language, self.java_kind)? else {
            unreachable!("ts handled by web delegate");
        };
        let registry = NativeRegistry::for_lane(
            self,
            &self.project_root,
            &lane,
            NativeRegistryOperation::ProjectResolve,
        )
        .await?;
        let resolution = registry
            .resolve(manifest, self.language == LibLanguage::Python)
            .await?;
        super::stage_pending_lock(&self.pending_lock, resolution.lock_packages)?;
        Ok(resolution.graph)
    }

    /// Resolve again while bypassing only the Web delegate's lock shortcut.
    /// Resolve lại và chỉ bỏ qua short-cut lock của Web delegate.
    pub(super) async fn resolve_native_dependencies_fresh(
        &self,
        manifest: &Manifest,
    ) -> MgResult<ResolvedGraph> {
        if let Some(web) = &self.web {
            web.arm_age_gate_for_core(&self.project_root, self.security_core)?;
            return web.resolve_fresh(manifest).await;
        }
        self.resolve_native_dependencies(manifest).await
    }
}

/// Map a Lib language and Java build manifest to its native resolver lane.
/// Ánh xạ ngôn ngữ Lib và build manifest Java sang lane resolver native.
pub(super) fn resolver_lane(
    language: LibLanguage,
    java_kind: JavaManifestKind,
) -> MgResult<Option<ResolveFirst>> {
    Ok(Some(match language {
        LibLanguage::Rust => ResolveFirst::Rust,
        LibLanguage::Python => ResolveFirst::Python,
        LibLanguage::Go => ResolveFirst::Go,
        LibLanguage::Java => match java_kind {
            JavaManifestKind::Pom => ResolveFirst::JavaPom,
            JavaManifestKind::Gradle | JavaManifestKind::None => {
                return Err(mgc_types::MgError::Unsupported {
                    core: "lib",
                    capability: "resolve",
                    guidance: "gradle build scripts are programs, not parseable manifests — \
                               declare dependencies in a pom.xml for native resolution, or \
                               let the gradle toolchain own resolution (mgc audits \
                               gradle/verification-metadata.xml)"
                        .to_string(),
                });
            }
            JavaManifestKind::Ambiguous => {
                return Err(mgc_types::MgError::Unsupported {
                    core: "lib",
                    capability: "resolve",
                    guidance: concat!(
                        "both Maven and Gradle manifests are present; ",
                        "select one build system before native dependency operations"
                    )
                    .to_string(),
                });
            }
        },
        LibLanguage::DotNet => ResolveFirst::DotNet,
        LibLanguage::Ts => return Ok(None),
    }))
}
