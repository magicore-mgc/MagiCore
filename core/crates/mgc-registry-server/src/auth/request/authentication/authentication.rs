//! Authorization header parsing.
// Parse Basic/Bearer và trích trusted token không đổi format.

use axum::http::HeaderMap;
use base64::Engine;

pub(in crate::auth) fn trusted_token_secret(headers: &HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?;
    if let Some(token) = value.strip_prefix("Bearer ") {
        return Some(token.to_owned());
    }
    let encoded = value.strip_prefix("Basic ")?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let (username, password) = decoded.split_once(':')?;
    (username == "mgc-oidc").then(|| password.to_owned())
}

/// Extract auth from request headers
pub fn extract_auth(headers: &HeaderMap) -> Option<(String, String)> {
    // Check Authorization header
    if let Some(auth) = headers.get("authorization") {
        let auth_str = auth.to_str().ok()?;
        if let Some(token) = auth_str.strip_prefix("Bearer ") {
            return Some(("Bearer".to_string(), token.to_string()));
        }
        if let Some(encoded) = auth_str.strip_prefix("Basic ")
            && let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(encoded)
        {
            let decoded_str = String::from_utf8(decoded).ok()?;
            let mut parts = decoded_str.splitn(2, ':');
            if let (Some(user), Some(pass)) = (parts.next(), parts.next()) {
                return Some((user.to_string(), pass.to_string()));
            }
        }
    }
    // Check for token in query (less secure, but supported)
    None
}
