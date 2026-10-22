//! Copy the supported dependency maps without rewriting package values.
//! Sao chép các map dependency được hỗ trợ mà không sửa giá trị package.

use serde_json::{Map, Value};

/// Return dependency groups used by the publish request body.
/// Trả về các nhóm dependency dùng trong request body publish.
pub fn dep_fields(manifest: &Value) -> Map<String, Value> {
    let mut out = Map::new();
    for key in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        if let Some(value) = manifest.get(key) {
            out.insert(key.into(), value.clone());
        }
    }
    out
}
