//! Dispatches manifest writing and Multi-project fallback in the existing order.
//! Dispatch ghi manifest và fallback Multi theo thứ tự hiện tại.

use std::path::Path;

use crate::language::AppLanguage;
use mgc_types::{Manifest, MgResult};

use super::super::{flutter, gradle, react_native, swift};

pub(in crate::manifest) fn run(
    language: AppLanguage,
    project_root: &Path,
    manifest: &Manifest,
) -> MgResult<()> {
    match language {
        AppLanguage::Flutter => flutter::write_pubspec(project_root, manifest),
        AppLanguage::Kotlin => gradle::write_build_gradle(project_root, manifest),
        AppLanguage::Swift => swift::write_package_swift(project_root, manifest),
        AppLanguage::ReactNative => react_native::write_package_json(project_root, manifest),
        AppLanguage::ObjC => flutter::write_podfile(project_root, manifest),
        AppLanguage::Multi => write_multi_manifest(project_root, manifest),
        AppLanguage::Maui | AppLanguage::Tauri => Err(mgc_types::MgError::Unsupported {
            core: "app",
            capability: "write-manifest",
            guidance: format!(
                "{} dependency manifest writes are not implemented by MagiCore",
                language.as_str()
            ),
        }),
    }
}

fn write_multi_manifest(project_root: &Path, manifest: &Manifest) -> MgResult<()> {
    if project_root.join("pubspec.yaml").exists() {
        return flutter::write_pubspec(project_root, manifest);
    }
    if project_root.join("build.gradle").exists() || project_root.join("build.gradle.kts").exists()
    {
        return gradle::write_build_gradle(project_root, manifest);
    }
    if project_root.join("Package.swift").exists() {
        return swift::write_package_swift(project_root, manifest);
    }
    if project_root.join("package.json").exists() {
        return react_native::write_package_json(project_root, manifest);
    }

    Err(mgc_types::MgError::Unsupported {
        core: "app",
        capability: "write-multi-manifest",
        guidance: "no supported app dependency manifest was detected; refusing to report a successful no-op write".to_string(),
    })
}
