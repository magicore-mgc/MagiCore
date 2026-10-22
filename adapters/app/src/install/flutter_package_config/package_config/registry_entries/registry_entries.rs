//! Builds and validates entries for resolved pub.dev packages.
//! Dựng và kiểm tra entry của package pub.dev đã resolve.

use mgc_types::ResolvedGraph;
use std::collections::BTreeSet;
use std::path::Path;

use super::super::super::types::FlutterPackageConfigEntry;
use super::super::super::version_floor::dart_language_version_floor;
use super::error::FlutterRegistryEntriesError;

pub(super) fn build_registry_entries_inner(
    graph: &ResolvedGraph,
    pub_cache: &Path,
    seen: &mut BTreeSet<String>,
) -> Result<Vec<FlutterPackageConfigEntry>, FlutterRegistryEntriesError> {
    let mut entries = Vec::with_capacity(graph.packages.len());
    for package in &graph.packages {
        let name = package.id.name_str();
        if !seen.insert(name.to_string()) {
            return Err(FlutterRegistryEntriesError::DuplicateName(name.to_string()));
        }
        let package_root = pub_cache.join("hosted").join("pub.dev").join(format!(
            "{}-{}",
            name,
            package.id.version()
        ));
        let manifest_path = package_root.join("pubspec.yaml");
        let manifest_text = std::fs::read_to_string(&manifest_path).map_err(|source| {
            FlutterRegistryEntriesError::ReadManifest {
                path: manifest_path.display().to_string(),
                source,
            }
        })?;
        let manifest: serde_yaml::Value =
            serde_yaml::from_str(&manifest_text).map_err(|source| {
                FlutterRegistryEntriesError::ParseManifest {
                    path: manifest_path.display().to_string(),
                    source,
                }
            })?;
        let actual_name = manifest
            .get("name")
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| {
                FlutterRegistryEntriesError::MissingManifestName(package_root.display().to_string())
            })?;
        if actual_name != name {
            return Err(FlutterRegistryEntriesError::ArchiveIdentity {
                graph: name.to_string(),
                archive: actual_name.to_string(),
            });
        }
        let language_version = manifest
            .get("environment")
            .and_then(|value| value.get("sdk"))
            .and_then(serde_yaml::Value::as_str)
            .and_then(dart_language_version_floor);
        let canonical_root = package_root
            .canonicalize()
            .map_err(FlutterRegistryEntriesError::CanonicalizeRoot)?;
        let root_uri = url::Url::from_directory_path(&canonical_root)
            .map_err(|_| FlutterRegistryEntriesError::EncodePackageRoot)?
            .to_string();
        entries.push(FlutterPackageConfigEntry {
            name: name.to_string(),
            root_uri,
            package_uri: "lib/",
            language_version,
        });
    }
    Ok(entries)
}
