#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! TOML string values round-trip without creating document syntax.
//! Giá trị chuỗi TOML round-trip mà không tạo cú pháp document.

use super::super::serialize_toml_string;

#[test]
fn serializer_escapes_syntax_and_preserves_the_original_value() {
    for value in [
        "demo",
        "device\"quoted",
        "triple \"\"\" quote",
        "path\\segment",
        "line\n[injected]\nowned = \"yes",
        "carriage\rreturn",
    ] {
        let encoded = serialize_toml_string(value);
        let document = format!("name={encoded}\n");
        let parsed: toml::Value = toml::from_str(&document).unwrap();

        assert_eq!(parsed["name"].as_str(), Some(value));
        assert!(parsed.get("injected").is_none());
    }

    assert_eq!(serialize_toml_string("demo"), "\"demo\"");
}
