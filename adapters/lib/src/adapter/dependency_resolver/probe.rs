//! Capability preflight for the native Lib resolver.
//! Preflight capability cho resolver native của Lib.

use crate::adapter::types::{JavaManifestKind, LibAdapter};
use crate::language::LibLanguage;
use mgc_types::MgResult;

impl LibAdapter {
    /// Reject resolver use when the current project lane is not natively owned.
    /// Từ chối resolver khi lane project hiện tại không thuộc quản lý native.
    pub(super) fn probe_native_dependency_resolver(&self) -> MgResult<()> {
        self.require_python_native("resolve")?;
        self.require_unique_dotnet_project("resolve")?;
        if self.language == LibLanguage::Java && self.java_kind != JavaManifestKind::Pom {
            let guidance = if self.java_kind == JavaManifestKind::Ambiguous {
                concat!(
                    "both Maven and Gradle manifests are present; ",
                    "select one build system before native dependency operations"
                )
            } else {
                "Java Gradle build scripts are not an MGC-native dependency manifest"
            };
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib", "resolve", guidance,
            ));
        }
        Ok(())
    }
}
