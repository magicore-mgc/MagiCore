//! Extract declared dependency names and requirements from a verified spec.
//! Trích tên dependency và requirement khai báo từ spec đã xác minh.

use serde_json::Value;

pub(in crate::protocols::reactnative::cocoapods) fn spec_dependencies(
    spec: &Value,
) -> Vec<(String, String)> {
    let Some(dependencies) = spec.get("dependencies").and_then(Value::as_object) else {
        return Vec::new();
    };
    dependencies
        .iter()
        .map(|(name, requirements)| {
            let range = requirements
                .as_array()
                .map(|values| {
                    values
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            (name.clone(), range)
        })
        .collect()
}

#[cfg(test)]
#[path = "test/dependencies.rs"]
mod tests;
