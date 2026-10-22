//! Trusted package write authorization.
use mgc_oidc::OidcClaims;
use std::time::Instant;
// Kiểm tra package, peer transport, repository binding và generation trước write.

use crate::auth::{AuthService, trusted_token_secret};
use crate::credential_security::TokenDigest;
use axum::http::{HeaderMap, StatusCode};
use std::net::SocketAddr;

impl AuthService {
    pub async fn authorize_package_write(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<Option<(OidcClaims, u64, String)>, StatusCode> {
        self.authorize_package_write_from(headers, package, None)
            .await
    }

    pub async fn authorize_package_write_from(
        &self,
        headers: &HeaderMap,
        package: &str,
        peer: Option<SocketAddr>,
    ) -> Result<Option<(OidcClaims, u64, String)>, StatusCode> {
        let binding = self
            .store
            .trusted_publisher_binding(package)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(token) = trusted_token_secret(headers) {
            let trusted = {
                let mut tokens = self.trusted_tokens_guard();
                tokens.retain(|_, trusted| trusted.expires_at > Instant::now());
                tokens
                    .get(TokenDigest::from_secret(&token).as_str())
                    .cloned()
            };
            if let Some(trusted) = trusted {
                if !self.trusted_transport_allowed(headers, peer) {
                    return Err(StatusCode::FORBIDDEN);
                }
                if trusted.package != package || !self.can_publish(&trusted.user, package) {
                    return Err(StatusCode::FORBIDDEN);
                }
                if binding.as_ref().is_none_or(|(repository, generation)| {
                    Some(repository.as_str()) != trusted.identity.repository.as_deref()
                        || *generation != trusted.binding_generation
                }) {
                    return Err(StatusCode::FORBIDDEN);
                }
                return Ok(Some((
                    trusted.identity.clone(),
                    trusted.binding_generation,
                    trusted.sigstore_oidc_token.clone(),
                )));
            }
        }
        if binding.is_some() {
            return Err(StatusCode::FORBIDDEN);
        }
        if self.admin_token.is_none() {
            return Ok(None);
        }
        let user = self
            .authenticate_headers_async(headers)
            .await
            .ok_or(StatusCode::UNAUTHORIZED)?;
        if !self.can_publish(&user, package) {
            return Err(StatusCode::FORBIDDEN);
        }
        Ok(None)
    }
}
