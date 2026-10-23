//! Insert baseline scripts only when the scaffold did not define them.
//! Chèn script baseline chỉ khi scaffold chưa định nghĩa.

use serde_json::{Map, Value};

/// Chèn mục scripts chỉ khi scaffold chưa định nghĩa — lựa chọn của
/// user/template thắng baseline mặc định.
pub(crate) fn ensure_script(root: &mut Map<String, Value>, name: &str, command: &str) {
    let scripts = root
        .entry("scripts".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(map) = scripts {
        map.entry(name.to_string())
            .or_insert_with(|| Value::String(command.to_string()));
    }
}
