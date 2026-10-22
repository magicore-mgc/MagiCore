use crate::adapter::read_dotnet_target_framework;
use crate::adapter::types::{JavaManifestKind, LibAdapter};
use crate::language::LibLanguage;
use anyhow::Result;
use std::path::Path;

impl LibAdapter {
    // P0-4 (2026-09-15): fallible construction — the TS lane builds a
    // WebAdapter whose registry-URL guard is a typed error now, so these
    // builders propagate Result instead of aborting.
    // (P0-4: dựng adapter có thể lỗi — lane TS dựng WebAdapter với guard
    // URL typed, builder propagate Result thay vì abort.)
    pub(in crate::adapter) fn for_language(
        language: LibLanguage,
        root: &Path,
        registry_url: Option<String>,
        token: Option<String>,
    ) -> Result<Self> {
        Self::for_language_with_chain(language, root, registry_url, token, &[])
    }

    pub(in crate::adapter) fn for_language_with_chain(
        language: LibLanguage,
        root: &Path,
        registry_url: Option<String>,
        token: Option<String>,
        fallbacks: &[(String, Option<String>)],
    ) -> Result<Self> {
        let web = if language == LibLanguage::Ts {
            Some(match (registry_url, token) {
                (Some(url), token) => mgc_web_adapter::WebAdapter::with_registry_chain(
                    url,
                    token,
                    fallbacks.to_vec(),
                )?,
                (None, _) => mgc_web_adapter::WebAdapter::new()?,
            })
        } else {
            None
        };
        Self::from_web(language, root, web)
    }

    pub(in crate::adapter) fn from_web(
        language: LibLanguage,
        root: &Path,
        web: Option<mgc_web_adapter::WebAdapter>,
    ) -> Result<Self> {
        // Java build-manifest kind: pom.xml is natively owned, gradle
        // build scripts fail closed downstream.
        // (Loại build-manifest Java: pom.xml sở hữu native, build script
        // gradle fail-closed phía sau.)
        let java_kind = match crate::language::dependency_manifest_format(root, language) {
            Some("maven-pom") => JavaManifestKind::Pom,
            Some("gradle") => JavaManifestKind::Gradle,
            Some("ambiguous") => JavaManifestKind::Ambiguous,
            _ => JavaManifestKind::None,
        };
        Ok(Self {
            language,
            security_core: "lib",
            java_kind,
            project_root: root.to_path_buf(),
            dotnet_tfm: read_dotnet_target_framework(root)?,
            web,
            pending_lock: std::sync::Mutex::new(Vec::new()),
        })
    }

    pub fn language(&self) -> &'static str {
        match self.language {
            LibLanguage::Ts => "ts",
            LibLanguage::Rust => "rust",
            LibLanguage::Python => "python",
            LibLanguage::Go => "go",
            LibLanguage::Java => "java",
            LibLanguage::DotNet => "dotnet",
        }
    }
}
