//! Trusted token cache lifecycle.
// Prune token hết hạn, lookup identity/binding và revoke token.

use crate::auth::{AuthService, TrustedToken, trusted_token_secret};
use crate::credential_security::TokenDigest;
use axum::http::HeaderMap;
use mgc_oidc::OidcClaims;
use std::collections::HashMap;
use std::sync::MutexGuard;
use std::time::Instant;

impl AuthService {
    pub(in crate::auth) fn has_active_trusted_token(&self, headers: &HeaderMap) -> bool {
        let Some(token) = trusted_token_secret(headers) else {
            return false;
        };
        let mut tokens = self.trusted_tokens_guard();
        tokens.retain(|_, value| value.expires_at > Instant::now());
        tokens.contains_key(TokenDigest::from_secret(&token).as_str())
    }

    pub(in crate::auth) fn trusted_tokens_guard(
        &self,
    ) -> MutexGuard<'_, HashMap<String, TrustedToken>> {
        self.trusted_tokens
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn trusted_identity(&self, headers: &HeaderMap, package: &str) -> Option<OidcClaims> {
        let token = trusted_token_secret(headers)?;
        let mut tokens = self.trusted_tokens_guard();
        tokens.retain(|_, value| value.expires_at > Instant::now());
        let trusted = tokens.get(TokenDigest::from_secret(&token).as_str())?;
        (trusted.package == package).then(|| trusted.identity.clone())
    }

    pub fn discard_trusted_token(&self, token: &str) {
        self.trusted_tokens_guard()
            .remove(TokenDigest::from_secret(token).as_str());
    }

    pub fn trusted_binding_generation(&self, headers: &HeaderMap, package: &str) -> Option<u64> {
        let token = trusted_token_secret(headers)?;
        let mut tokens = self.trusted_tokens_guard();
        tokens.retain(|_, trusted| trusted.expires_at > Instant::now());
        let trusted = tokens.get(TokenDigest::from_secret(&token).as_str())?;
        (trusted.package == package).then_some(trusted.binding_generation)
    }
}
