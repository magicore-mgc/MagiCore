//! Per-operation ownership projection — projection ownership theo operation.
//! Every output row comes directly from the dependency gate.
//! Mỗi hàng output lấy trực tiếp từ dependency gate.

use serde_json::{Map, Value};

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, owner_for};

/// Project all operation decisions for one core/language/manifest context.
/// Chiếu quyết định của mọi operation cho một context core/language/manifest.
pub(in crate::commands::capabilities::dependency_ownership) fn operations_for(
    core: &str,
    ecosystem: Option<&str>,
    manifest_format: Option<&str>,
) -> Value {
    let mut map = Map::new();
    for op in DepOp::ALL {
        let ctx =
            DepContext::new(core, ecosystem, None, None, *op).with_manifest_format(manifest_format);
        let (owner, tools): (&str, Vec<String>) = match owner_for(&ctx) {
            DepOwner::Native => ("mgc-native", Vec::new()),
            DepOwner::ScaffoldOnly => ("scaffold-only", Vec::new()),
            DepOwner::Unsupported => ("unsupported", Vec::new()),
        };
        let mut cell = serde_json::json!({"owner": owner, "tools": tools});
        if matches!(
            (core, ecosystem),
            ("game", Some("bevy")) | ("iot", Some("esp32-rust"))
        ) && owner == "mgc-native"
        {
            cell["requires"] = serde_json::json!("Cargo.toml");
        }
        map.insert(op.as_str().to_string(), cell);
    }
    Value::Object(map)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
