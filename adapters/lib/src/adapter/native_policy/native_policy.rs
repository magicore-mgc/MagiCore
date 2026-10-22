use crate::adapter::types::{JavaManifestKind, LibAdapter, ResolveFirst};
use crate::language::{LibLanguage, find_csproj};
use mgc_types::MgResult;
use std::path::Path;

impl LibAdapter {
    pub(in crate::adapter) fn python_native_owned(&self) -> bool {
        self.language != LibLanguage::Python
            || crate::manifest::supports_native_python_project(&self.project_root)
    }

    pub(in crate::adapter) fn require_unique_dotnet_project(
        &self,
        operation: &'static str,
    ) -> MgResult<()> {
        if self.language == LibLanguage::DotNet && find_csproj(&self.project_root).is_none() {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                operation,
                "native .NET dependency operations require exactly one regular root-level .csproj; select a project explicitly",
            ));
        }
        Ok(())
    }

    pub(in crate::adapter) fn require_python_native(
        &self,
        capability: &'static str,
    ) -> MgResult<()> {
        self.require_python_native_at(&self.project_root, capability)
    }

    pub(in crate::adapter) fn require_python_native_at(
        &self,
        project_root: &Path,
        capability: &'static str,
    ) -> MgResult<()> {
        if self.language == LibLanguage::Python
            && !crate::manifest::supports_native_python_project(project_root)
        {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                capability,
                "this Python project uses dependency sources or lockfiles not owned by the MGC native PEP 621 lane; migrate explicitly or use a separately selected compatibility manager",
            ));
        }
        Ok(())
    }

    /// Which native resolve-first lane owns this project (`Ok(None)` = the
    /// embedded web delegate owns resolution, i.e. TS). Java-gradle fails
    /// closed: build scripts are programs, not parseable manifests.
    /// (Lane native resolve-first; TS do web delegate; gradle fail-closed.)
    pub(in crate::adapter) fn resolve_first_lane(&self) -> MgResult<Option<ResolveFirst>> {
        self.require_python_native("resolve")?;
        if self.web.is_some() {
            return Ok(None);
        }
        Ok(match self.language {
            LibLanguage::Python => Some(ResolveFirst::Python),
            LibLanguage::Rust => Some(ResolveFirst::Rust),
            LibLanguage::Go => Some(ResolveFirst::Go),
            LibLanguage::DotNet => Some(ResolveFirst::DotNet),
            LibLanguage::Java => match self.java_kind {
                JavaManifestKind::Pom => Some(ResolveFirst::JavaPom),
                // pom.xml is natively owned; gradle scripts are programs —
                // fail closed instead of booking a mutation the disk never
                // sees (write_manifest is a no-op there by design).
                JavaManifestKind::Gradle | JavaManifestKind::None => {
                    return Err(mgc_types::MgError::Other(
                        "java add needs a pom.xml — gradle build scripts are programs, not parseable manifests (declare dependencies in a pom.xml for native add)"
                            .to_string(),
                    ));
                }
                JavaManifestKind::Ambiguous => {
                    return Err(mgc_types::MgError::Other(
                        "java project contains both pom.xml and Gradle build scripts; select one build system before native dependency operations"
                            .to_string(),
                    ));
                }
            },
            LibLanguage::Ts => None,
        })
    }
}
