//! Parse Flutter pubspec into MGC's app manifest.
//! Parse pubspec Flutter thành manifest app của MGC.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::{Ecosystem, Manifest};
use std::path::Path;

/// Parse pubspec.yaml to Manifest.
pub(in crate::manifest::flutter::flutter) fn parse_pubspec(
    project_root: &Path,
) -> FlutterResult<Manifest> {
    let pubspec_path = project_root.join("pubspec.yaml");
    let content = mgc_adapter_base::project_file::read_regular_text(&pubspec_path, "pubspec.yaml")?;

    let pubspec: super::super::types::PubspecYaml =
        serde_yaml::from_str(&content).map_err(|source| FlutterError::YamlParse {
            context: "failed to parse pubspec.yaml".to_string(),
            source,
        })?;

    if pubspec
        .dependency_overrides
        .as_ref()
        .is_some_and(|overrides| !overrides.is_empty())
    {
        return Err(FlutterError::Unsupported {
            capability: "Flutter dependency_overrides",
            guidance: "MagiCore does not yet implement pub dependency override semantics; remove dependency_overrides before native resolution".to_string(),
        });
    }
    if pubspec
        .workspace
        .as_ref()
        .is_some_and(|members| !members.is_empty())
    {
        return Err(FlutterError::Unsupported {
            capability: "Flutter pub workspace resolution",
            guidance: "MagiCore does not yet resolve Dart pub workspaces as one graph; run this operation only after workspace support is implemented".to_string(),
        });
    }

    let mut manifest = Manifest::new(&pubspec.name, Ecosystem::App);

    // Parse dependencies
    if let Some(deps) = pubspec.dependencies {
        super::super::dependency::parse_dependency_section(&mut manifest, deps, false)?;
    }

    if let Some(dev_deps) = pubspec.dev_dependencies {
        super::super::dependency::parse_dependency_section(&mut manifest, dev_deps, true)?;
    }

    Ok(manifest)
}
