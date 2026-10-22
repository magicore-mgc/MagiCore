//! Verify JWT signature and registered claims — xác minh chữ ký và claim JWT.
//! Không trả lại token; kết quả chỉ chứa identity đã xác minh.

use super::super::{EXPIRY_LEEWAY_SECS, Jwks, MAX_JWT_BYTES, MAX_TOKEN_LIFETIME_SECS};
use crate::claims::OidcClaims;
use crate::error::OidcError;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use std::time::{SystemTime, UNIX_EPOCH};

/// Validate `token`: RS256 only, `kid`-pinned key, issuer + audience +
/// expiry enforced. Returns the verified identity — never the token.
/// Xác minh token bằng RS256, khóa theo `kid`, issuer, audience và hạn dùng; chỉ trả identity đã xác minh.
pub fn validate(
    token: &str,
    allowed_issuers: &[&str],
    audience: &str,
    jwks: &Jwks,
) -> Result<OidcClaims, OidcError> {
    if token.trim().is_empty()
        || token.len() > MAX_JWT_BYTES
        || audience.is_empty()
        || allowed_issuers.is_empty()
    {
        return Err(OidcError::Malformed("empty token".to_string()));
    }
    let header = decode_header(token)
        .map_err(|error| OidcError::Malformed(format!("bad JWT header: {error}")))?;
    if header.alg != Algorithm::RS256 {
        return Err(OidcError::VerificationFailed(format!(
            "unsupported JWT algorithm {:?} (RS256 only)",
            header.alg
        )));
    }
    let kid = header
        .kid
        .ok_or_else(|| OidcError::VerificationFailed("JWT has no kid".to_string()))?;
    let jwk = jwks
        .keys
        .iter()
        .find(|key| key.kid.as_deref() == Some(kid.as_str()))
        .ok_or_else(|| {
            OidcError::VerificationFailed("JWT kid matches no trusted key".to_string())
        })?;
    if jwk.kty != "RSA" {
        return Err(OidcError::VerificationFailed(format!(
            "unsupported key type '{}' (RSA only)",
            jwk.kty
        )));
    }
    let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|error| OidcError::VerificationFailed(format!("unusable trusted key: {error}")))?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);
    validation.set_issuer(allowed_issuers);
    validation.leeway = EXPIRY_LEEWAY_SECS;
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
    let data = decode::<OidcClaims>(token, &key, &validation).map_err(|error| {
        use jsonwebtoken::errors::ErrorKind;
        match error.kind() {
            ErrorKind::ExpiredSignature => {
                OidcError::VerificationFailed("OIDC token expired".to_string())
            }
            ErrorKind::InvalidAudience => {
                OidcError::VerificationFailed("OIDC audience mismatch".to_string())
            }
            ErrorKind::InvalidIssuer => {
                OidcError::VerificationFailed("OIDC issuer mismatch".to_string())
            }
            ErrorKind::InvalidSignature => {
                OidcError::VerificationFailed("OIDC signature invalid".to_string())
            }
            _ => OidcError::VerificationFailed(format!("OIDC token rejected: {error}")),
        }
    })?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| OidcError::VerificationFailed("system clock precedes Unix epoch".into()))?
        .as_secs();
    if data.claims.iat == 0
        || data.claims.iat > now.saturating_add(EXPIRY_LEEWAY_SECS)
        || data.claims.exp <= data.claims.iat
        || data.claims.exp.saturating_sub(data.claims.iat) > MAX_TOKEN_LIFETIME_SECS
    {
        return Err(OidcError::VerificationFailed(
            "invalid OIDC issuance time".into(),
        ));
    }
    Ok(data.claims)
}
