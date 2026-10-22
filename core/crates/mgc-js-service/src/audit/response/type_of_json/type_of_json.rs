// JSON kind used in schema diagnostics. / Loại JSON dùng trong diagnostic schema.
/// Human-readable JSON value kind for schema error messages.
/// Tên loại giá trị JSON cho thông báo lỗi schema.
pub(in crate::audit) fn type_of_json(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}
