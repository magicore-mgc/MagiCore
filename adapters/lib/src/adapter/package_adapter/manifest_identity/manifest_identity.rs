//! Stable project manifest identity mapping for Lib languages.
//! Ánh xạ identity manifest ổn định theo ngôn ngữ Lib.

use crate::adapter::types::{JavaManifestKind, LibAdapter};
use crate::language::LibLanguage;

pub(in crate::adapter::package_adapter) fn manifest_identity(
    adapter: &LibAdapter,
) -> Option<mgc_types::ManifestIdentity> {
    let (language, format, relpath) = match adapter.language {
        LibLanguage::Ts => ("ts", "package.json", "package.json"),
        LibLanguage::Rust => ("rust", "Cargo.toml", "Cargo.toml"),
        LibLanguage::Python => ("python", "pyproject.toml", "pyproject.toml"),
        LibLanguage::Go => ("go", "go.mod", "go.mod"),
        LibLanguage::Java => match adapter.java_kind {
            JavaManifestKind::Pom => ("java", "pom.xml", "pom.xml"),
            JavaManifestKind::Gradle => ("java", "build.gradle", "build.gradle"),
            JavaManifestKind::Ambiguous => ("java", "ambiguous", ""),
            JavaManifestKind::None => ("java", "pom.xml", "pom.xml"),
        },
        LibLanguage::DotNet => ("dotnet", "csproj", "*.csproj"),
    };
    Some(mgc_types::ManifestIdentity {
        core: "lib".to_string(),
        language: language.to_string(),
        format: format.to_string(),
        relpath: relpath.to_string(),
    })
}
