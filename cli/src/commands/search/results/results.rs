//! Map registry objects into CLI results and apply the optional core filter.
//! Ánh xạ object registry thành kết quả CLI và áp dụng bộ lọc core tùy chọn.

use serde_json::Value;

use super::super::{classification::detect_core_label, types::SearchResult};

pub(in crate::commands::search) fn map_and_filter_results(
    objects: &[Value],
    core_filter: Option<&str>,
) -> Vec<SearchResult> {
    objects
        .iter()
        .map(|object| {
            let package = &object["package"];
            let name = package["name"].as_str().unwrap_or("?").to_string();
            let version = package["version"].as_str().unwrap_or("?").to_string();
            let description = package["description"].as_str().unwrap_or("").to_string();
            let score = object["score"]["final"]
                .as_f64()
                .map(|value| format!("{:.3}", value))
                .unwrap_or_default();
            let core_label = detect_core_label(&name, &description);

            SearchResult {
                name,
                version,
                description,
                score,
                core_label,
            }
        })
        .filter(|result| {
            // Keep the existing filter normalization and substring matching behavior.
            // Giữ nguyên cách chuẩn hóa label và so khớp chuỗi con hiện tại.
            if let Some(filter) = core_filter {
                result.core_label.to_lowercase().contains(filter)
            } else {
                true
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
