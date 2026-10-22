//! npmrc public data types / Type dữ liệu public của npmrc.

use std::collections::HashMap;
use std::fmt;

/// npm registry configuration and credentials.
/// Cấu hình registry npm và credential.
#[derive(Clone, Default)]
pub struct NpmRc {
    /// registry=URL (registry mặc định)
    /// URL registry mặc định.
    pub registry: Option<String>,
    /// @scope:registry=URL — registry riêng theo scope
    /// Registry riêng cho package scope.
    pub scope_registries: HashMap<String, String>,
    /// //host/:_authToken=TOKEN
    /// Token theo host và path; không hiển thị giá trị khi Debug.
    pub auth_tokens: HashMap<String, String>,
    /// //host/:username + //host/:_password (base64)
    /// Basic auth theo host; password được lưu dạng base64 như npmrc.
    pub basic_auth: HashMap<String, (String, String)>,
}

/// A borrowed token and its selected HTTP scope; no Debug token output.
/// Token được mượn cùng HTTP scope đã chọn; không in token qua Debug.
pub struct NpmCredential<'a> {
    /// Secret token borrowed from the npmrc model.
    /// Token bí mật được mượn từ model npmrc.
    pub token: &'a String,
    /// Registry URL scope the token is allowed to reach.
    /// Scope URL registry mà token được phép gửi tới.
    pub scope: url::Url,
}

impl fmt::Debug for NpmRc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NpmRc")
            .field("registry", &self.registry)
            .field("scope_registries", &self.scope_registries)
            .field(
                "auth_token_hosts",
                &self.auth_tokens.keys().collect::<Vec<_>>(),
            )
            .field(
                "basic_auth_hosts",
                &self.basic_auth.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
