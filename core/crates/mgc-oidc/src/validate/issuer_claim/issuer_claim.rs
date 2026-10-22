//! Read one JWT payload claim without treating it as authenticated — đọc claim payload nhưng không coi là đã xác thực.
//!
use crate::error::OidcError;
use base64::Engine as _;

/// Read `iss` only to select one configured JWKS endpoint; signature checks still establish trust.
/// Chỉ đọc `iss` để chọn một JWKS endpoint đã cấu hình; chữ ký vẫn là căn cứ xác thực duy nhất.
pub(crate) fn unverified_issuer(token: &str) -> Result<String, OidcError> {
    let payload = token
        .split('.')
        .nth(1)
        .filter(|payload| !payload.is_empty())
        .ok_or_else(|| OidcError::Malformed("JWT has no claims payload".into()))?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| OidcError::Malformed("JWT claims payload is not base64url".into()))?;
    let claims: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| OidcError::Malformed("JWT claims payload is not valid JSON".into()))?;
    claims
        .get("iss")
        .and_then(serde_json::Value::as_str)
        .filter(|issuer| !issuer.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| OidcError::Malformed("JWT has no issuer claim".into()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
