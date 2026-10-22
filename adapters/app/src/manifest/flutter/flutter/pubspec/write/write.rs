//! Safely write pinned dependencies back into pubspec.yaml.
//! Ghi an toàn dependency đã pin lại vào pubspec.yaml.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::Manifest;
use std::path::Path;

/// Write Manifest back to pubspec.yaml (native add/update). Rebuilds
/// the `dependencies` map from the manifest (exact pins stay exact);
/// every other key is preserved. A non-exact range fails closed —
/// callers pin via resolve-first. Git/path deps absent from the manifest
/// are dropped honestly (mgc manages versioned pins, not source refs).
/// (Ghi pubspec.yaml từ manifest.)
pub(in crate::manifest::flutter::flutter) fn write_pubspec(
    project_root: &Path,
    manifest: &Manifest,
) -> FlutterResult<()> {
    let pubspec_path = project_root.join("pubspec.yaml");
    let content = mgc_adapter_base::project_file::read_regular_text(&pubspec_path, "pubspec.yaml")?;
    let mut doc: serde_yaml::Value =
        serde_yaml::from_str(&content).map_err(|source| FlutterError::YamlParse {
            context: "parse pubspec.yaml".to_string(),
            source,
        })?;
    // Validate the entire existing dependency shape before reconstructing it.
    // Kiểm tra toàn bộ cấu trúc dependency cũ trước khi dựng lại.
    let _current_manifest = super::super::parse_pubspec(project_root)?;
    // Refuse source-specific entries before rebuilding maps; otherwise a direct
    // writer caller could silently erase Git/path/hosted semantics.
    // Từ chối source đặc thù trước khi dựng lại map để không xóa semantics.
    if let Some(root) = doc.as_mapping() {
        for section in ["dependencies", "dev_dependencies"] {
            let Some(entries) = root
                .get(serde_yaml::Value::String(section.to_string()))
                .and_then(serde_yaml::Value::as_mapping)
            else {
                continue;
            };
            for (name, value) in entries {
                let Some(fields) = value.as_mapping() else {
                    continue;
                };
                let flutter_sdk = fields.len() == 1
                    && fields
                        .get(serde_yaml::Value::String("sdk".to_string()))
                        .and_then(serde_yaml::Value::as_str)
                        == Some("flutter");
                if !flutter_sdk {
                    let name = name.as_str().unwrap_or("<non-string>");
                    return Err(FlutterError::Unsupported {
                        capability: "rewrite Flutter source dependency",
                        guidance: format!(
                            "dependency '{name}' in {section} uses source-specific metadata; MagiCore refuses to discard it during native manifest mutation"
                        ),
                    });
                }
            }
        }
    }
    let mut deps = serde_yaml::Mapping::new();
    let mut dev_deps = serde_yaml::Mapping::new();
    // Flutter SDK entries are supplied by the SDK, not version pins — retain
    // them in both sections so add/remove cannot erase flutter_test or
    // integration_test. Các SDK dependency không phải pin; giữ nguyên cả hai nhóm.
    if let Some(root) = doc.as_mapping() {
        for (section, target) in [
            ("dependencies", &mut deps),
            ("dev_dependencies", &mut dev_deps),
        ] {
            if let Some(original) = root
                .get(serde_yaml::Value::String(section.to_string()))
                .and_then(serde_yaml::Value::as_mapping)
            {
                for (name, value) in original {
                    let is_flutter_sdk = value
                        .as_mapping()
                        .and_then(|fields| fields.get(serde_yaml::Value::String("sdk".to_string())))
                        .and_then(serde_yaml::Value::as_str)
                        == Some("flutter");
                    if is_flutter_sdk {
                        target.insert(name.clone(), value.clone());
                    }
                }
            }
        }
    }
    let mut pins: Vec<(String, String, bool)> = Vec::new();
    for dep in manifest.all_dependencies() {
        let version = dep
            .range
            .satisfying_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| dep.range.to_string());
        let clean = version.trim_start_matches(['=', 'v', ' ']);
        mgc_types::Version::parse(clean).map_err(|_| {
            FlutterError::Other(format!(
                "pubspec writer needs an exact version for '{}', got '{}' — resolve-first must pin before save",
                dep.name.as_str(),
                dep.range
            ))
        })?;
        // Pinned manifest deps only (the SDK block was carried above).
        if dep.name.as_str() == "flutter" {
            continue;
        }
        pins.push((dep.name.as_str().to_string(), clean.to_string(), dep.dev));
    }
    pins.sort();
    pins.dedup();
    for (name, version, is_dev) in pins {
        // `^x.y.z` is Dart's caret-implied default (like `flutter pub add`).
        let target = if is_dev { &mut dev_deps } else { &mut deps };
        target.insert(
            serde_yaml::Value::String(name),
            serde_yaml::Value::String(format!("^{version}")),
        );
    }
    if let Some(map) = doc.as_mapping_mut() {
        map.insert(
            serde_yaml::Value::String("dependencies".to_string()),
            serde_yaml::Value::Mapping(deps),
        );
        if !dev_deps.is_empty() {
            map.insert(
                serde_yaml::Value::String("dev_dependencies".to_string()),
                serde_yaml::Value::Mapping(dev_deps),
            );
        }
    }
    let rendered =
        serde_yaml::to_string(&doc).map_err(|source| FlutterError::YamlSerialize { source })?;
    mgc_adapter_base::project_file::atomic_write_regular(&pubspec_path, rendered.as_bytes())?;
    Ok(())
}
