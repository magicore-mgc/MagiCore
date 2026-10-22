//! NuGet v3 service-index resources and paginated registration leaves.
//! Resource service-index NuGet v3 và registration leaves phân trang.

use crate::protocols::nuget::NuGetProtocol;
use mgc_types::{MgError, MgResult};
use serde_json::Value;

impl NuGetProtocol {
    /// Registration leaves (flattened across pages) as raw JSON values.
    /// Các leaf registration (dẹt qua mọi page) dưới dạng JSON thô.
    pub(in crate::protocols::nuget) async fn registration_leaves(
        &self,
        id: &str,
    ) -> MgResult<Vec<Value>> {
        let url = format!(
            "{}/{}/index.json",
            self.registrations_base,
            id.to_lowercase()
        );
        let (status, body) = self.get_text(&url).await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {url} returned {status} — no registration for {id} (fail-closed)"
            )));
        }
        let doc: Value = serde_json::from_str(&body)
            .map_err(|e| MgError::Other(format!("parse registration index failed: {e}")))?;
        let mut leaves = Vec::new();
        collect_leaves(&doc, &mut leaves);
        // Paginated hives (most real packages: pages carry `@id` links and
        // NO inline leaves) — follow each page URL and collect its leaves.
        // Fail-closed on page fetch errors: a half-read index would resolve
        // against an incomplete version set.
        // (Hive phân trang (hầu hết package thật) — theo URL từng page.)
        if let Some(pages) = doc.get("items").and_then(Value::as_array) {
            for page in pages {
                let has_inline = page
                    .get("items")
                    .and_then(Value::as_array)
                    .is_some_and(|items| !items.is_empty());
                if has_inline {
                    continue;
                }
                let Some(page_url) = page.get("@id").and_then(Value::as_str) else {
                    continue;
                };
                let (page_status, page_body) = self.get_text(page_url).await?;
                if !(200..300).contains(&page_status) {
                    return Err(MgError::Network(format!(
                        "GET {page_url} returned {page_status} — registration page unreadable, index incomplete (fail-closed)"
                    )));
                }
                let page_doc: Value = serde_json::from_str(&page_body)
                    .map_err(|e| MgError::Other(format!("parse registration page failed: {e}")))?;
                collect_leaves(&page_doc, &mut leaves);
            }
        }
        Ok(leaves)
    }
}

/// Find a resource `@id` by `@type` in a service index document.
/// Tìm resource `@id` theo `@type` trong tài liệu service index.
pub(in crate::protocols::nuget) fn find_resource(doc: &Value, ty: &str) -> Option<String> {
    doc.get("resources")?.as_array()?.iter().find_map(|r| {
        let matches = r.get("@type").and_then(Value::as_str) == Some(ty);
        if matches {
            r.get("@id")
                .and_then(Value::as_str)
                .map(|s| s.trim_end_matches('/').to_string())
        } else {
            None
        }
    })
}

/// Flatten registration leaves: a leaf has `catalogEntry`; pages/intermediate
/// nodes carry `items` arrays.
/// Dẹt leaf registration: leaf có `catalogEntry`; page/node trung gian mang
/// mảng `items`.
pub(in crate::protocols::nuget) fn collect_leaves(value: &Value, out: &mut Vec<Value>) {
    match value {
        Value::Array(items) => items.iter().for_each(|v| collect_leaves(v, out)),
        Value::Object(obj) => {
            if obj.contains_key("catalogEntry") {
                out.push(value.clone());
                return;
            }
            if let Some(items) = obj.get("items") {
                collect_leaves(items, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
