use anyhow::Result;

pub(in crate::commands::config) fn display_config_value<'a>(key: &str, value: &'a str) -> &'a str {
    // SEC-17: a key name is a hint, not evidence. `.npmrc` stores credentials
    // under `//host/:_authToken`, and a project may name a token field
    // anything; the value's shape is what actually identifies a secret.
    // (SEC-17: tên key chỉ là gợi ý, không phải bằng chứng. `.npmrc` lưu
    // credential dưới `//host/:_authToken`, và project có thể đặt tên field
    // token bất kỳ; hình dạng của giá trị mới là thứ nhận diện secret.)
    if is_sensitive(key) || value_looks_secret(value) {
        "***"
    } else {
        value
    }
}

/// Whether a value's shape marks it as a credential.
/// Hình dạng của giá trị có đánh dấu nó là credential không.
///
/// Covers the three spellings npm credentials actually take: a scoped
/// `_authToken` line, an `Authorization` bearer value, and a long opaque
/// base64/hex blob that is not a port, path or version.
/// (Bao ba dạng credential npm thật dùng: dòng `_authToken` theo scope, giá trị
/// bearer trong `Authorization`, và blob base64/hex dài không phải port, path
/// hay version.)
pub(in crate::commands::config) fn value_looks_secret(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lowered = trimmed.to_ascii_lowercase();
    if lowered.starts_with("bearer ") || lowered.starts_with("basic ") {
        return true;
    }
    if lowered.starts_with("//") && lowered.contains("_authtoken") {
        return true;
    }
    // `mgc_` prefixed registration tokens are self-identifying (SEC-16).
    // (Token đăng ký có prefix `mgc_` tự nhận diện — SEC-16.)
    if lowered.starts_with("mgc_") {
        return true;
    }
    // A long opaque run with no separators is a key, not a value people read
    // (ports, paths and versions all contain punctuation or stay short).
    // (Chuỗi dài không phân cách là key, không phải giá trị để đọc — port, path
    // và version đều có dấu hoặc ngắn.)
    let opaque = trimmed.len() >= 32
        && trimmed.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c == '-' || c == '_'
        })
        && trimmed.chars().any(|c| c.is_ascii_digit())
        && trimmed.chars().filter(|c| c.is_ascii_alphabetic()).count() >= 16;
    if opaque {
        return true;
    }
    false
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
