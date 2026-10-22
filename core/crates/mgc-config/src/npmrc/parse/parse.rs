//! npmrc line parsing and environment expansion / Parse dòng npmrc và biến môi trường.

use anyhow::Result;

use crate::npmrc::NpmRc;

impl NpmRc {
    /// Parse supported npmrc entries and expand environment placeholders.
    /// Parse entry npmrc được hỗ trợ và khai triển biến môi trường.
    pub fn parse(content: &str) -> Result<Self> {
        let mut rc = Self::default();
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = expand_env(value.trim());

            if key == "registry" {
                rc.registry = Some(value);
            } else if let Some(scope) = key.strip_suffix(":registry") {
                // @scope:registry=URL
                if scope.starts_with('@') {
                    rc.scope_registries.insert(scope.to_string(), value);
                }
            } else if let Some(host) = key.strip_suffix(":_authToken") {
                let host = Self::normalize_host(host);
                rc.auth_tokens.insert(host, value);
            } else if let Some(host) = key.strip_suffix(":username") {
                let host = Self::normalize_host(host);
                let (name, _) = rc.basic_auth.entry(host).or_default();
                *name = value;
            } else if let Some(host) = key.strip_suffix(":_password") {
                let host = Self::normalize_host(host);
                let (name, pass) = rc.basic_auth.entry(host).or_default();
                if name.is_empty() {
                    // password trước username: tạm lưu, username set sau → pass giữ
                    *pass = value;
                } else {
                    *pass = value;
                }
            }
            // key khác (cache, always-auth...) — bỏ qua
        }
        Ok(rc)
    }
}

/// Env expansion: `${VAR}` + `$VAR` → giá trị env (nếu có; không có → giữ nguyên)
fn expand_env(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find('$') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        if let Some(braced) = after.strip_prefix('{') {
            // ${VAR} form
            if let Some(end) = braced.find('}') {
                out.push_str(&env_lookup(&braced[..end]));
                rest = &braced[end + 1..];
                continue;
            }
        }
        if let Some(end) = after.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            out.push_str(&env_lookup(&after[..end]));
            rest = &after[end..];
        } else {
            out.push_str(&env_lookup(after));
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

fn env_lookup(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
