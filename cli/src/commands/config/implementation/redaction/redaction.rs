use anyhow::Result;

pub(in crate::commands::config) fn display_config_value<'a>(key: &str, value: &'a str) -> &'a str {
    if is_sensitive(key) { "***" } else { value }
}

pub(in crate::commands::config) fn redact_toml_config(content: &str) -> Result<String> {
    fn redact_value(value: &mut toml::Value) {
        match value {
            toml::Value::Table(table) => {
                for (key, nested) in table.iter_mut() {
                    if is_sensitive(key) {
                        *nested = toml::Value::String("***".to_string());
                    } else {
                        redact_value(nested);
                    }
                }
            }
            toml::Value::Array(values) => {
                for nested in values {
                    redact_value(nested);
                }
            }
            _ => {}
        }
    }

    let mut value: toml::Value = toml::from_str(content)?;
    redact_value(&mut value);
    Ok(toml::to_string_pretty(&value)?)
}

pub(in crate::commands::config) fn is_sensitive(key: &str) -> bool {
    let mut normalized = String::with_capacity(key.len() + 4);
    let mut previous = None;
    let mut chars = key.chars().peekable();
    while let Some(character) = chars.next() {
        if !character.is_ascii_alphanumeric() {
            if !normalized.is_empty() && !normalized.ends_with('_') {
                normalized.push('_');
            }
            previous = Some(character);
            continue;
        }

        let camel_boundary = character.is_ascii_uppercase()
            && (previous
                .is_some_and(|prev: char| prev.is_ascii_lowercase() || prev.is_ascii_digit())
                || (previous.is_some_and(|prev: char| prev.is_ascii_uppercase())
                    && chars.peek().is_some_and(|next| next.is_ascii_lowercase())));
        if camel_boundary && !normalized.is_empty() && !normalized.ends_with('_') {
            normalized.push('_');
        }
        normalized.push(character.to_ascii_lowercase());
        previous = Some(character);
    }
    while normalized.ends_with('_') {
        normalized.pop();
    }
    normalized.contains("token")
        || normalized.contains("password")
        || normalized.contains("secret")
        || normalized == "auth"
        || normalized.ends_with("_auth")
        || normalized.contains("authorization")
        || (normalized.ends_with("_key") && !normalized.ends_with("_public_key"))
        // Short password spellings (`pass`, `passwd`, `pwd`) — matched as
        // whole words or suffixes only, so `bypass_proxy` stays visible.
        // (Dạng viết tắt password — chỉ khớp từ đầy đủ/hậu tố.)
        || normalized == "pass"
        || normalized.ends_with("_pass")
        || normalized.contains("passwd")
        || normalized == "pwd"
        || normalized.ends_with("_pwd")
        // A bare `key` alone is a secret name; longer words containing it
        // (`monkey`, `keyboard`) are not.
        || normalized == "key"
}
