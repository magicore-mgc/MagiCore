//! Escapes TOML basic strings without locale or platform dependence.
//! Escape basic string TOML không phụ thuộc locale hay nền tảng.

/// TOML basic-string escaping (spec-conformant, locale-independent).
/// Escape chuỗi basic TOML (đúng spec, không phụ thuộc locale).
pub(in crate::canonical) fn escape_toml_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() + 2);
    out.push('"');
    for ch in raw.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04X}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
