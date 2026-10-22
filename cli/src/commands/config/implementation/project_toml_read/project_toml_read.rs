use super::super::redact_toml_config;
use crate::commands::config::implementation::is_sensitive;
use std::path::Path;

/// Đọc giá trị từ mgc.toml theo key đơn hoặc dot-notation (vd: "game.engine")
pub(in crate::commands::config) fn toml_value(toml_path: &Path, key: &str) -> Option<String> {
    if is_sensitive(key) {
        return Some("***".to_string());
    }
    let content = mgc_adapter_base::project_file::read_regular_text(toml_path, "mgc.toml").ok()?;
    // Redact nested credentials too: requesting a whole table must not
    // bypass the key-level protection applied to `config list`.
    // (Che cả credential lồng nhau để lấy nguyên bảng không lách được `config list`.)
    let redacted = redact_toml_config(&content).ok()?;
    let doc: toml_edit::DocumentMut = redacted.parse().ok()?;
    let parts: Vec<&str> = key.splitn(2, '.').collect();
    if parts.len() == 2 {
        let val = doc.get(parts[0])?.as_table_like()?.get(parts[1])?;
        Some(toml_val_to_string(val))
    } else {
        let val = doc.get(key)?;
        Some(toml_val_to_string(val))
    }
}

fn toml_val_to_string(v: &toml_edit::Item) -> String {
    match v {
        toml_edit::Item::Value(val) => val
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| val.as_integer().map(|i| i.to_string()))
            .or_else(|| val.as_bool().map(|b| b.to_string()))
            .or_else(|| val.as_float().map(|f| f.to_string()))
            .unwrap_or_else(|| val.to_string()),
        other => other.to_string(),
    }
}
