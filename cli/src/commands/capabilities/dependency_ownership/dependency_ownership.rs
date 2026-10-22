//! Dependency ownership manifest — manifest dependency ownership.
//! Composes operation, language and manifest projections from the dep gate.
//! Ghép projection operation, language và manifest từ dep gate.

use serde_json::{Map, Value};

use super::language_projection::language_projection;
use super::manifest_variants::{ai_manifest_variants, lib_manifest_variants};
use super::operation_projection::operations_for;

/// Dependency ownership per core, derived from the C0 firewall table
/// (`dep_gate::owner_for` over full `DepContext`s) — the SINGLE
/// machine-readable source the lifecycle matrix cross-checks against
/// (T0.4). Shape per operation: {"owner", "tools"}; cores with
/// per-ecosystem splits additionally carry a "languages" map with ONLY
/// the differing ecosystems (gate ids: "rn", not "react-native").
/// Quyền sở hữu dependency từng core, suy ra từ bảng tường lửa C0 —
/// nguồn máy-đọc DUY NHẤT matrix cross-check (T0.4).
pub fn dependency_ownership(core: &str) -> serde_json::Value {
    let base = operations_for(core, None, None);
    let languages = language_projection(core, &base);
    let mut root = Map::new();
    root.insert("operations".to_string(), base);
    if !languages.is_empty() {
        root.insert("languages".to_string(), Value::Object(languages));
    }
    if core == "lib" {
        root.insert("manifest_variants".to_string(), lib_manifest_variants());
    }
    if core == "ai" {
        root.insert(
            "manifest_requirements".to_string(),
            serde_json::json!({"python": "mgc-pyproject"}),
        );
        root.insert("manifest_variants".to_string(), ai_manifest_variants());
    }
    Value::Object(root)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
