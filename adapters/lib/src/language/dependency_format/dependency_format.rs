//! Native dependency-manifest ownership by language.
//! Quyền sở hữu native dependency manifest theo ngôn ngữ.

use std::path::Path;

use crate::language::{LibLanguage, find_csproj, is_regular_manifest};

/// Return the native dependency format owned by this language lane.
/// Trả format dependency native mà lane ngôn ngữ sở hữu.
pub fn dependency_manifest_format(root: &Path, language: LibLanguage) -> Option<&'static str> {
    match language {
        LibLanguage::Ts if is_regular_manifest(&root.join("package.json")) => Some("package-json"),
        LibLanguage::Rust if is_regular_manifest(&root.join("Cargo.toml")) => Some("cargo-toml"),
        LibLanguage::Go if is_regular_manifest(&root.join("go.mod")) => Some("go-mod"),
        LibLanguage::Python if is_regular_manifest(&root.join("pyproject.toml")) => {
            if crate::manifest::supports_native_python_project(root) {
                Some("pep621-native")
            } else {
                Some("pyproject-unsupported")
            }
        }
        LibLanguage::Java => {
            let has_pom = is_regular_manifest(&root.join("pom.xml"));
            let has_gradle = is_regular_manifest(&root.join("build.gradle"))
                || is_regular_manifest(&root.join("build.gradle.kts"));
            match (has_pom, has_gradle) {
                (true, false) => Some("maven-pom"),
                (false, true) => Some("gradle"),
                (true, true) => Some("ambiguous"),
                (false, false) => None,
            }
        }
        LibLanguage::DotNet if find_csproj(root).is_some() => Some("csproj"),
        LibLanguage::Ts
        | LibLanguage::Rust
        | LibLanguage::Python
        | LibLanguage::Go
        | LibLanguage::DotNet => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
