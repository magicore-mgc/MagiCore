//! Writes scalar and array TOML fields using the canonical escaper.
//! Ghi field scalar và mảng TOML bằng bộ escaper canonical.

use crate::canonical::toml_escaping::escape_toml_string;
pub(in crate::canonical) fn push_str_field(out: &mut String, key: &str, value: &str) {
    out.push_str(key);
    out.push_str(" = ");
    out.push_str(&escape_toml_string(value));
    out.push('\n');
}

pub(in crate::canonical) fn push_string_array(out: &mut String, key: &str, values: &[String]) {
    out.push_str(key);
    out.push_str(" = [");
    let mut first = true;
    for value in values {
        if !first {
            out.push_str(", ");
        }
        first = false;
        out.push_str(&escape_toml_string(value));
    }
    out.push_str("]\n");
}
