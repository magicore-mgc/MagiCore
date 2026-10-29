//! Credential verification with protected cache keys and bounded async KDF work.
//! Verify credential bằng cache digest và KDF async có giới hạn.

use crate::auth::{AuthService, User, UserRole};
use crate::credential_security::{TokenDigest, constant_time_eq, run_kdf, verify_password_hash};
use axum::http::HeaderMap;
use base64::Engine;
use std::time::Instant;

impl AuthService {
    fn password_candidate(&self, username: &str) -> Option<(String, String)> {
        self.users_guard()
            .iter()
            .find(|(_, user)| user.name == username)
            .and_then(|(key, user)| {
                user.password
                    .as_ref()
                    .map(|hash| (key.clone(), hash.clone()))
            })
    }

    fn verified_password_user(&self, key: &str, hash: &str) -> Option<User> {
        let users = self.users_guard();
        let current = users.get(key)?;
        let unchanged = current
            .password
            .as_deref()
            .is_some_and(|stored| constant_time_eq(stored, hash));
        unchanged.then(|| current.clone())
    }

    /// Synchronous library compatibility API; HTTP uses the async variant.
    /// API thư viện đồng bộ tương thích; HTTP dùng biến thể async.
    pub fn verify_password(&self, username: &str, password: &str) -> Option<User> {
        let (key, hash) = self.password_candidate(username)?;
        if !verify_password_hash(password, &hash) {
            return None;
        }
        self.verified_password_user(&key, &hash)
    }

    pub async fn verify_password_async(&self, username: &str, password: &str) -> Option<User> {
        let (key, hash) = self.password_candidate(username)?;
        let expected = hash.clone();
        let password = password.to_owned();
        let matched = run_kdf(move || Ok(verify_password_hash(&password, &hash)))
            .await
            .ok()?;
        if !matched {
            return None;
        }
        // Recheck revocation/password rotation after the worker returns.
        // Kiểm lại revoke/rotate password sau khi worker hoàn tất.
        self.verified_password_user(&key, &expected)
    }

    fn trusted_token_user(&self, key: &str) -> Option<User> {
        let mut tokens = self.trusted_tokens_guard();
        tokens.retain(|_, value| value.expires_at > Instant::now());
        tokens.get(key).map(|value| value.user.clone())
    }

    fn is_admin_secret(&self, secret: &str) -> bool {
        let candidate = TokenDigest::from_secret(secret);
        self.admin_token
            .as_deref()
            .is_some_and(|stored| constant_time_eq(stored, candidate.as_str()))
    }

    /// Client tokens remain opaque plaintext on the wire, never stored as cache keys.
    /// Token client vẫn opaque trên wire, không được lưu nguyên làm key cache.
    pub fn verify_token(&self, token: &str) -> Option<User> {
        if token.trim().is_empty() {
            return None;
        }
        let key = TokenDigest::from_secret(token);
        if self
            .admin_token
            .as_deref()
            .is_some_and(|stored| constant_time_eq(stored, key.as_str()))
        {
            return Some(User {
                name: "admin".to_string(),
                is_admin: true,
                role: UserRole::Admin,
                scopes: vec![],
                password: None,
                email: None,
            });
        }
        let regular = self.users_guard().get(key.as_str()).cloned();
        regular.or_else(|| self.trusted_token_user(key.as_str()))
    }

    /// Synchronous library API; request handlers use authenticate_headers_async.
    /// API thư viện đồng bộ; handler request dùng authenticate_headers_async.
    pub fn authenticate_headers(&self, headers: &HeaderMap) -> Option<User> {
        let value = headers.get("authorization")?.to_str().ok()?;
        if let Some(token) = value.strip_prefix("Bearer ") {
            return self.verify_token(token);
        }
        let (name, password) = basic_credentials(value)?;
        if name == "mgc-oidc" {
            return self.trusted_token_user(TokenDigest::from_secret(&password).as_str());
        }
        if self.is_admin_secret(&password) {
            return self.verify_token(&password);
        }
        self.verify_password(&name, &password)
    }

    pub async fn authenticate_headers_async(&self, headers: &HeaderMap) -> Option<User> {
        let value = headers.get("authorization")?.to_str().ok()?;
        if let Some(token) = value.strip_prefix("Bearer ") {
            return self.verify_token(token);
        }
        let (name, password) = basic_credentials(value)?;
        if name == "mgc-oidc" {
            return self.trusted_token_user(TokenDigest::from_secret(&password).as_str());
        }
        if self.is_admin_secret(&password) {
            return self.verify_token(&password);
        }
        self.verify_password_async(&name, &password).await
    }
}

fn basic_credentials(value: &str) -> Option<(String, String)> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value.strip_prefix("Basic ")?)
        .ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let (name, password) = decoded.split_once(':')?;
    Some((name.to_owned(), password.to_owned()))
}
