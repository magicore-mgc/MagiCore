//! Parse machine-readable output from `swift package dump-package`.
//! Parse đầu ra dạng máy đọc được từ `swift package dump-package`.
use crate::protocols::swift::types::SwiftDep;
use mgc_types::{MgError, MgResult};
use serde_json::Value;

/// Parse `swift package dump-package` JSON output (tolerant — the source
/// array shape varies across toolchain versions).
/// Parse JSON output của `swift package dump-package` (khoan dung — hình
/// dạng mảng source đổi theo version toolchain).
pub fn parse_dump_package(json: &str) -> MgResult<Vec<SwiftDep>> {
    let doc: Value = serde_json::from_str(json)
        .map_err(|e| MgError::Other(format!("parse dump-package JSON failed: {e}")))?;
    let mut deps = Vec::new();
    for dep in doc
        .get("dependencies")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        // New shape (Swift 5.7+, what real toolchains emit):
        // `{"registry": [{"identity": ..., "requirement": {...}}]}` and
        // `{"remote": [{"url": ..., ...}]}`.
        // Dạng mới (Swift 5.7+, toolchain thực tế phát ra): registry có identity/requirement hoặc remote có URL.
        if let Some(items) = dep.get("registry").and_then(Value::as_array) {
            for item in items {
                let identity = item
                    .get("identity")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let requirement = dump_requirement(item.get("requirement"));
                push_dump_dep(&mut deps, "registry", identity, requirement);
            }
            continue;
        }
        if let Some(items) = dep.get("remote").and_then(Value::as_array) {
            for item in items {
                let url = item.get("url").and_then(Value::as_str).unwrap_or_default();
                let requirement = dump_requirement(item.get("requirement"));
                push_dump_dep(&mut deps, "remote", url, requirement);
            }
            continue;
        }
        // Legacy shape: `{"source": ["registry"|"remote", target, req]}`.
        // Dạng cũ dùng mảng source chứa loại nguồn, đích và requirement.
        let Some(source) = dep.get("source").and_then(Value::as_array) else {
            continue;
        };
        let Some(kind) = source.first().and_then(Value::as_str) else {
            continue;
        };
        let target = source.get(1).and_then(Value::as_str).unwrap_or_default();
        let requirement = dump_requirement(source.get(2));
        push_dump_dep(&mut deps, kind, target, requirement);
    }
    Ok(deps)
}

/// Push one dump-package dep (shared by the new registry/remote shapes
/// and the legacy source-array shape).
/// Thêm một dependency dump-package dùng chung cho dạng registry/remote mới và mảng source cũ.
fn push_dump_dep(deps: &mut Vec<SwiftDep>, kind: &str, target: &str, requirement: Option<String>) {
    let Some(requirement) = requirement else {
        // Unmappable requirement — honest skip (cannot pin
        // deterministically).
        // (Requirement không ánh xạ được — skip trung thực (không ghim
        // tất định được).)
        eprintln!(
            "WARNING: dump-package dependency '{target}' has an unmappable requirement — skipped"
        );
        return;
    };
    match kind {
        "registry" => deps.push(SwiftDep::Registry {
            identity: target.to_string(),
            requirement,
        }),
        "remote" => deps.push(SwiftDep::Git {
            url: target.to_string(),
            requirement,
        }),
        // local/editing deps are toolchain-owned paths — never resolved
        // against a registry.
        // (dep local/editing là path do toolchain giữ — không bao giờ
        // resolve qua registry.)
        other => eprintln!(
            "WARNING: dump-package dependency '{target}' source '{other}' is not registry-resolvable — skipped"
        ),
    }
}

/// Requirement object from dump-package → graph range string. `None` = the
/// shape is not understood (caller skips honestly).
/// Object requirement từ dump-package → chuỗi range graph. `None` = hình
/// dạng không hiểu (caller skip trung thực).
fn dump_requirement(value: Option<&Value>) -> Option<String> {
    let value = value?;
    if let Some(v) = value.get("from").and_then(Value::as_str) {
        return Some(format!("from:{v}"));
    }
    if let Some(ranges) = value.get("ranges").and_then(Value::as_array) {
        let parts: Vec<String> = ranges
            .iter()
            .filter_map(|r| {
                // String form (`"1.0.0..<2.0.0"`) or bound objects
                // (`{"lowerBound": "1.0.0", "upperBound": "2.0.0"}` —
                // the shape real `dump-package` emits).
                // Dạng range có thể là chuỗi hoặc object lowerBound/upperBound.
                if let Some(s) = r.as_str() {
                    return Some(format!("range:{}", s.trim()));
                }
                let lo = r.get("lowerBound").and_then(Value::as_str)?;
                let hi = r.get("upperBound").and_then(Value::as_str)?;
                Some(format!("range:{lo}..<{hi}"))
            })
            .collect();
        if !parts.is_empty() {
            return Some(parts.join("||"));
        }
    }
    if let Some(r) = value.get("range").and_then(Value::as_str) {
        return Some(format!("range:{}", r.trim()));
    }
    // Singular `range` holding bound objects (the shape real
    // `dump-package` emits: `"range": [{"lowerBound": "1.0.0",
    // "upperBound": "2.0.0"}]`).
    // Trường `range` đơn có các cận lowerBound/upperBound như output dump-package thực tế.
    if let Some(arr) = value.get("range").and_then(Value::as_array) {
        let parts: Vec<String> = arr
            .iter()
            .filter_map(|r| {
                let lo = r.get("lowerBound").and_then(Value::as_str)?;
                let hi = r.get("upperBound").and_then(Value::as_str)?;
                Some(format!("range:{lo}..<{hi}"))
            })
            .collect();
        if !parts.is_empty() {
            return Some(parts.join("||"));
        }
    }
    if let Some(v) = value.get("exact").and_then(Value::as_str) {
        return Some(format!("exact:{v}"));
    }
    if let Some(v) = value.get("branch").and_then(Value::as_str) {
        return Some(format!("branch:{v}"));
    }
    if let Some(v) = value.get("revision").and_then(Value::as_str) {
        return Some(format!("revision:{v}"));
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
