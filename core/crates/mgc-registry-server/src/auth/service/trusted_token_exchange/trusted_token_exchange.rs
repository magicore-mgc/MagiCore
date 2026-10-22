//! OIDC trusted token exchange.
// Validate OIDC/Sigstore identity, repository binding, quota và TTL trước khi cấp token.

use crate::auth::{AuthService, TrustedToken, User, UserRole};
use crate::credential_security::TokenDigest;
use crate::trusted::{
    MAX_ACTIVE_TRUSTED_TOKENS, TRUSTED_TOKEN_TTL_SECS, claims_match_repository_binding,
};
use anyhow::Result;
use axum::http::StatusCode;
use mgc_oidc::OidcClaims;
use std::time::{Duration, Instant};

impl AuthService {
    pub async fn exchange_trusted_token(
        &self,
        package: &str,
        jwt: &str,
        sigstore_jwt: &str,
    ) -> Result<(String, u64, OidcClaims), StatusCode> {
        let config = self.trusted_config.as_ref().ok_or(StatusCode::NOT_FOUND)?;
        if !self.trusted_enabled() {
            return Err(StatusCode::NOT_FOUND);
        }
        let issuers = config
            .issuers
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let claims = mgc_oidc::validate::validate_with_discovery(jwt, &issuers, &config.audience)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let sigstore_claims =
            mgc_oidc::validate::validate_with_discovery(sigstore_jwt, &issuers, "sigstore")
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
        if !claims.same_workload_identity(&sigstore_claims) {
            return Err(StatusCode::FORBIDDEN);
        }
        let (repository, binding_generation) = self
            .store
            .trusted_publisher_binding(package)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::FORBIDDEN)?;
        if !claims_match_repository_binding(&claims, &repository) {
            return Err(StatusCode::FORBIDDEN);
        }

        let token = format!(
            "{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        );
        let user = User {
            name: format!("oidc:{}", repository),
            is_admin: false,
            role: UserRole::Publisher,
            scopes: vec![package.to_string()],
            password: None,
            email: None,
        };
        let mut tokens = self.trusted_tokens_guard();
        tokens.retain(|_, token| token.expires_at > Instant::now());
        if tokens.len() >= MAX_ACTIVE_TRUSTED_TOKENS {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
        tokens.insert(
            TokenDigest::from_secret(&token).into_string(),
            TrustedToken {
                user,
                package: package.to_string(),
                identity: claims.clone(),
                sigstore_oidc_token: sigstore_jwt.to_owned(),
                binding_generation,
                expires_at: Instant::now() + Duration::from_secs(TRUSTED_TOKEN_TTL_SECS),
            },
        );
        Ok((token, TRUSTED_TOKEN_TTL_SECS, claims))
    }
}
