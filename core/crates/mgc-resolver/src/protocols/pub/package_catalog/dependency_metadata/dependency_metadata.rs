//! Interpret PubSpec dependency values and preserve explicit SDK/source policy.
//! Đọc giá trị dependency PubSpec, giữ rõ policy SDK và nguồn package.

use super::super::error::CatalogError;
use super::super::package_name::validate_pub_package_name;
use mgc_types::MgResult;
use std::collections::HashMap;

pub(in crate::protocols::pubdev::package_catalog) fn parse_pub_dependencies(
    dependencies: &HashMap<String, serde_json::Value>,
    markers: &mut Vec<String>,
) -> MgResult<Vec<(String, String)>> {
    let mut parsed = Vec::with_capacity(dependencies.len());
    for (name, value) in dependencies {
        validate_pub_package_name(name)?;
        // SDK-owned packages are part of the selected SDK, not pub.dev.
        if matches!(
            name.as_str(),
            "flutter" | "flutter_test" | "integration_test" | "dart"
        ) {
            markers.push(format!("sdk-owned:{name}"));
            continue;
        }
        let Some(range) = value.as_str() else {
            let source = value
                .as_object()
                .and_then(|object| {
                    ["path", "git", "hosted"]
                        .into_iter()
                        .find(|key| object.contains_key(*key))
                })
                .unwrap_or("non-registry");
            return Err(CatalogError::UnsupportedDependencySource {
                name: name.clone(),
                source_kind: source.to_string(),
            }
            .into_mg_error());
        };
        if range.trim().is_empty() {
            return Err(CatalogError::EmptyConstraint { name: name.clone() }.into_mg_error());
        }
        parsed.push((name.clone(), range.to_string()));
    }
    parsed.sort();
    Ok(parsed)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
