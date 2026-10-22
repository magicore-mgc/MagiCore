//! Render a string value with the pinned TOML serializer.
//! Render giá trị chuỗi bằng TOML serializer đã khóa.

pub(in crate::scaffold) fn serialize_toml_string(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}
