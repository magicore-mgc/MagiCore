//! Route an untrusted JWT claim only to an allowlisted issuer before verification — chỉ định tuyến claim chưa xác minh tới issuer được allowlist.
//!
use crate::claims::OidcClaims;
use crate::error::OidcError;
use crate::validate::{MAX_CACHED_ISSUERS, MAX_JWT_BYTES};
use crate::validate::{cached_jwks, unverified_issuer, validate};
use jsonwebtoken::{Algorithm, decode_header};

/// Validate with cached issuer discovery; only configured issuers are fetched.
/// Cache discovery/JWKS; chỉ kết nối issuer được cấu hình từ phía server.
pub async fn validate_with_discovery(
    token: &str,
    allowed_issuers: &[&str],
    audience: &str,
) -> Result<OidcClaims, OidcError> {
    if token.len() > MAX_JWT_BYTES || allowed_issuers.len() > MAX_CACHED_ISSUERS {
        return Err(OidcError::Malformed("OIDC input exceeds size limit".into()));
    }
    let header =
        decode_header(token).map_err(|_| OidcError::Malformed("invalid JWT header".into()))?;
    if header.alg != Algorithm::RS256 {
        return Err(OidcError::VerificationFailed("RS256 is required".into()));
    }
    let kid = header
        .kid
        .filter(|kid| !kid.is_empty())
        .ok_or_else(|| OidcError::Malformed("JWT has no kid".into()))?;
    let token_issuer = unverified_issuer(token)?;
    let issuer = allowed_issuers
        .iter()
        .copied()
        .find(|configured| *configured == token_issuer)
        .ok_or_else(|| {
            OidcError::UntrustedIssuer("issuer is not in the configured allowlist".into())
        })?;
    let keys = cached_jwks(issuer, &kid).await?;
    validate(token, &[issuer], audience, &keys)
}
