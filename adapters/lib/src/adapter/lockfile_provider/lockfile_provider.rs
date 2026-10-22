use crate::adapter::types::{JavaManifestKind, LibAdapter};
use crate::language::LibLanguage;
use crate::manifest::{
    write_cargo_manifest, write_csproj_manifest, write_go_mod_manifest, write_pom_manifest,
    write_pyproject_manifest,
};
use async_trait::async_trait;
use mgc_types::capabilities::LockfileProvider;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl LockfileProvider for LibAdapter {
    /// Evidence: real manifest writers — cargo/pyproject (crate::manifest)
    /// and the web delegate for TS. Dẫn chứng: bộ viết manifest thật —
    /// cargo/pyproject (crate::manifest) và web delegate cho TS.
    fn probe_lockfile_provider(&self) -> MgResult<()> {
        self.require_python_native("write_manifest")?;
        if self.language == LibLanguage::Java && self.java_kind != JavaManifestKind::Pom {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "write_manifest",
                "Java Gradle build scripts are not an MGC-owned manifest",
            ));
        }
        Ok(())
    }

    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        self.require_python_native_at(project_root, "write_manifest")?;
        if let Some(web) = &self.web {
            return web.write_manifest(project_root, manifest).await;
        }
        match self.language {
            LibLanguage::Rust => write_cargo_manifest(project_root, manifest),
            LibLanguage::Python => write_pyproject_manifest(project_root, manifest),
            // mgc owns the go.mod require set (native add) — every other
            // directive is preserved verbatim by the writer.
            // (mgc sở hữu require go.mod — directive khác giữ nguyên.)
            LibLanguage::Go => write_go_mod_manifest(project_root, manifest),
            // mgc owns csproj PackageReferences (native add) — the rest
            // of the project file is preserved verbatim by the writer.
            // (mgc sở hữu PackageReference — phần còn lại giữ nguyên.)
            LibLanguage::DotNet => write_csproj_manifest(project_root, manifest),
            // mgc owns pom.xml dependencies (native add) — but ONLY for
            // pom projects; gradle build scripts are programs and stay
            // read-only: fail closed instead of reporting a phantom
            // write (a silent Ok here would fake remove success).
            // (mgc sở hữu dependency pom.xml — gradle chỉ đọc, fail rõ.)
            LibLanguage::Java => match self.java_kind {
                JavaManifestKind::Pom => write_pom_manifest(project_root, manifest),
                JavaManifestKind::Gradle | JavaManifestKind::None => Err(mgc_types::MgError::Other(
                    "refusing to write: gradle build scripts are programs, not manifests (declare dependencies in a pom.xml for mgc-managed edits)".to_string(),
                )),
                JavaManifestKind::Ambiguous => Err(mgc_types::MgError::Other(
                    "refusing to write: both Maven and Gradle manifests are present; select one build system before native dependency operations".to_string(),
                )),
            },
            LibLanguage::Ts => unreachable!("ts handled by web delegate"),
        }
    }
}
