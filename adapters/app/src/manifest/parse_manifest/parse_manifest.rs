//! Dispatches manifest parsing and Multi-project fallback in the existing order.
//! Dispatch parse manifest và fallback Multi theo thứ tự hiện tại.

use std::path::Path;

use crate::language::AppLanguage;
use mgc_types::{Manifest, MgResult};

use super::super::{flutter, gradle, react_native, swift};

pub(in crate::manifest) fn run(language: AppLanguage, project_root: &Path) -> MgResult<Manifest> {
    match language {
        AppLanguage::Flutter => flutter::parse_pubspec(project_root),
        AppLanguage::Kotlin => gradle::parse_build_gradle(project_root),
        AppLanguage::Swift => swift::parse_package_swift(project_root),
        AppLanguage::ReactNative => react_native::parse_package_json(project_root),
        AppLanguage::ObjC => flutter::parse_podfile(project_root),
        AppLanguage::Multi => parse_multi_manifest(project_root),
        AppLanguage::Maui | AppLanguage::Tauri => Err(mgc_types::MgError::Unsupported {
            core: "app",
            capability: "parse-manifest",
            guidance: format!(
                "{} is detected as an app project, but MagiCore does not yet parse its dependency manifest",
                language.as_str()
            ),
        }),
    }
}

fn parse_multi_manifest(project_root: &Path) -> MgResult<Manifest> {
    if project_root.join("pubspec.yaml").exists() {
        return flutter::parse_pubspec(project_root);
    }
    if project_root.join("build.gradle").exists() || project_root.join("build.gradle.kts").exists()
    {
        return gradle::parse_build_gradle(project_root);
    }
    if project_root.join("Package.swift").exists() {
        return swift::parse_package_swift(project_root);
    }
    if project_root.join("package.json").exists() {
        return react_native::parse_package_json(project_root);
    }

    Err(mgc_types::MgError::Unsupported {
        core: "app",
        capability: "parse-multi-manifest",
        guidance: "no supported app dependency manifest was detected; refusing to return an empty dependency graph".to_string(),
    })
}
