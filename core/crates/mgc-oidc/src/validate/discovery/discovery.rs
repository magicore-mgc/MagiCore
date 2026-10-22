//! Discover an issuer JWKS endpoint and fetch its keys — tìm endpoint JWKS và tải khóa.
//! Các request dùng chung transport HTTPS, timeout, giới hạn body và policy redirect.

use super::super::jwks::Jwks;
use crate::error::OidcError;
use crate::fetch::{fetch_json, secure_url};

/// Fetch a JWKS document (bounded timeout — no hanging publishes).
/// Tải JWKS với timeout giới hạn để publish không bị treo.
pub async fn fetch_jwks(jwks_url: &str) -> Result<Jwks, OidcError> {
    serde_json::from_value(fetch_json(secure_url(jwks_url)?, None).await?)
        .map_err(|_| OidcError::FetchFailed("invalid JWKS document".into()))
}

/// OpenID discovery: `{issuer}/.well-known/openid-configuration` → `jwks_uri`.
/// Dùng OpenID discovery để lấy `jwks_uri` từ issuer.
pub async fn discover_jwks_url(issuer: &str) -> Result<String, OidcError> {
    let url = format!(
        "{}/.well-known/openid-configuration",
        issuer.trim_end_matches('/')
    );
    let issuer_url = secure_url(issuer)?;
    let body = fetch_json(secure_url(&url)?, None).await?;
    if body.get("issuer").and_then(|value| value.as_str()) != Some(issuer) {
        return Err(OidcError::VerificationFailed(
            "discovery issuer mismatch".into(),
        ));
    }
    let jwks = body
        .get("jwks_uri")
        .and_then(|value| value.as_str())
        .ok_or_else(|| OidcError::FetchFailed("discovery has no jwks_uri".into()))?;
    let jwks_url = secure_url(jwks)?;
    if jwks_url.origin() != issuer_url.origin() {
        return Err(OidcError::VerificationFailed(
            "cross-origin JWKS discovery is not supported".into(),
        ));
    }
    Ok(jwks_url.to_string())
}
