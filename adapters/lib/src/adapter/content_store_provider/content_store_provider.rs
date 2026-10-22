use crate::adapter::types::JavaManifestKind;
use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use async_trait::async_trait;
use mgc_lockfile::EcosystemTag;
use mgc_types::ContentStoreProvider;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

fn lock_ecosystem(language: LibLanguage) -> Option<EcosystemTag> {
    match language {
        LibLanguage::Rust => Some(EcosystemTag::Rust),
        LibLanguage::Python => Some(EcosystemTag::Python),
        LibLanguage::Go => Some(EcosystemTag::Go),
        LibLanguage::Java => Some(EcosystemTag::Maven),
        LibLanguage::DotNet => Some(EcosystemTag::NuGet),
        LibLanguage::Ts => None,
    }
}

#[async_trait]
impl ContentStoreProvider for LibAdapter {
    /// Evidence: crate::install::run_install with the shared store
    /// (install/shared_store.rs — mgc-managed caches per toolchain).
    /// Dẫn chứng: crate::install::run_install với shared store
    /// (install/shared_store.rs — cache do mgc quản theo toolchain).
    fn probe_content_store(&self) -> MgResult<()> {
        self.require_python_native("install")?;
        if self.language == LibLanguage::Java && self.java_kind != JavaManifestKind::Pom {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "install",
                "this Java build lane has no MGC-native dependency graph",
            ));
        }
        Ok(())
    }

    async fn install(
        &self,
        graph: &ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        self.require_python_native_at(project_root, "install")?;
        if self.language == LibLanguage::Java && self.java_kind != JavaManifestKind::Pom {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "install",
                "Java Gradle build scripts are not an MGC-native dependency manifest",
            ));
        }
        // Use new install pipeline (install/mod.rs)
        // Dùng install pipeline mới (install/mod.rs)
        let mut lock_packages = std::mem::take(&mut *self.pending_lock.lock().map_err(|_| {
            mgc_types::MgError::Other(
                "lib pending lock poisoned — refusing rather than operating on uncertain state"
                    .to_string(),
            )
        })?);
        if crate::install::needs_existing_lock_rows(graph, &lock_packages) {
            // Delta operations (add after an existing lock) stage markers
            // only for their newly resolved subgraph. Fill the rest from
            // the existing exact lock pins; lock-short-circuit installs use
            // the same path. Never install a graph entry without its own
            // matching lock/integrity record.
            // (Delta resolve chỉ stage subgraph mới; bổ sung entry còn lại
            // từ lock hiện hữu đúng version.)
            // A complete fresh resolve does not borrow replay pins from an obsolete graph.
            // Resolve mới đầy đủ không mượn pin replay từ đồ thị cũ không còn hợp lệ.
            if let Some(ecosystem) = lock_ecosystem(self.language) {
                let existing = crate::install::read_existing_lock(project_root)?;
                crate::install::complete_lock_packages_from_existing(
                    graph,
                    ecosystem,
                    &mut lock_packages,
                    &existing,
                );
            }
        }
        crate::install::run_install(
            self.language,
            self.web.as_ref(),
            graph,
            project_root,
            opts,
            None, // Issue #6: pass ContentStore when available
            lock_packages,
        )
        .await
    }
}
