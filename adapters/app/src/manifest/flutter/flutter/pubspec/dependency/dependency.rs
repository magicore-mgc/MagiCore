//! Convert pubspec dependency declarations and reject unsupported sources.
//! Chuyển khai báo dependency pubspec và từ chối nguồn chưa hỗ trợ.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::{DependencySpec, Manifest, PackageName, VersionRange};
use std::collections::HashMap;

/// Parse pubspec version value (can be string or object).
pub(in crate::manifest::flutter::flutter::pubspec) fn parse_dependency_section(
    manifest: &mut Manifest,
    dependencies: HashMap<String, serde_yaml::Value>,
    dev: bool,
) -> FlutterResult<()> {
    for (name, value) in dependencies {
        if let serde_yaml::Value::Mapping(mapping) = &value {
            let sdk = mapping
                .get(serde_yaml::Value::String("sdk".to_string()))
                .and_then(serde_yaml::Value::as_str);
            if let Some(sdk) = sdk {
                if mapping.len() != 1 {
                    return Err(FlutterError::Unsupported {
                        capability: "mixed Flutter SDK dependency source",
                        guidance: format!(
                            "dependency '{name}' combines an SDK source with other fields; MagiCore refuses to guess its resolution"
                        ),
                    });
                }
                // Flutter SDK packages are supplied by the selected SDK,
                // then inserted into MGC's generated package_config.json.
                if !sdk.is_empty() {
                    if sdk == "flutter" {
                        continue;
                    }
                    return Err(FlutterError::Unsupported {
                        capability: "non-Flutter SDK dependency",
                        guidance: format!(
                            "dependency '{name}' uses SDK '{sdk}', which MagiCore cannot bind into its native package configuration"
                        ),
                    });
                }
            }

            let source = ["path", "git", "hosted"]
                .into_iter()
                .find(|key| mapping.contains_key(serde_yaml::Value::String((*key).to_string())))
                .unwrap_or("non-registry");
            return Err(FlutterError::Unsupported {
                capability: "Flutter non-pub.dev dependency source",
                guidance: format!(
                    "dependency '{name}' uses {source}; MagiCore currently resolves only pub.dev registry dependencies and will not substitute a same-name pub.dev package"
                ),
            });
        }

        let range = match value {
            serde_yaml::Value::String(range) if !range.trim().is_empty() => range,
            serde_yaml::Value::Null => "*".to_string(),
            _ => {
                return Err(FlutterError::Unsupported {
                    capability: "Flutter dependency constraint",
                    guidance: format!(
                        "dependency '{name}' has an unsupported pubspec value; expected a version string or an SDK dependency"
                    ),
                });
            }
        };
        let package = PackageName::new(&name).map_err(|error| FlutterError::Unsupported {
            capability: "Flutter dependency name",
            guidance: format!("MagiCore cannot safely resolve dependency '{name}': {error}"),
        })?;
        let range = VersionRange::parse(&range).map_err(|error| FlutterError::Unsupported {
            capability: "Flutter dependency range",
            guidance: format!("MagiCore cannot safely resolve '{name}': {error}"),
        })?;
        manifest.add_dep(DependencySpec::new(package, range), dev, false, false);
    }
    Ok(())
}
