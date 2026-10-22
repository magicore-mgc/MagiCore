//! Resolve one configured env/file token pair — đọc một cặp nguồn token env/file.
//!
use crate::error::OidcError;

/// Env var carrying a pre-minted OIDC JWT (generic CI, tests, escape hatch).
/// Biến môi trường chứa JWT OIDC đã cấp sẵn cho CI chung hoặc kiểm thử.
pub const OIDC_TOKEN_ENV: &str = "MGC_OIDC_TOKEN";
/// File carrying a pre-minted OIDC JWT (some CI systems mount it).
/// File chứa JWT OIDC đã cấp sẵn trên các CI mount token vào file.
pub const OIDC_TOKEN_FILE_ENV: &str = "MGC_OIDC_TOKEN_FILE";
/// Explicit OIDC token with the public Sigstore audience.
/// Token OIDC riêng có audience Sigstore công khai.
pub const SIGSTORE_OIDC_TOKEN_ENV: &str = "MGC_SIGSTORE_OIDC_TOKEN";
/// File carrying an OIDC token with the public Sigstore audience.
/// File chứa OIDC token riêng có audience Sigstore công khai.
pub const SIGSTORE_OIDC_TOKEN_FILE_ENV: &str = "MGC_SIGSTORE_OIDC_TOKEN_FILE";
fn normalize_token(value: String) -> Option<String> {
    let token = value.trim().to_owned();
    (!token.is_empty()).then_some(token)
}

pub(crate) fn token_from_environment(
    token_env: &str,
    file_env: &str,
    file_error_prefix: &str,
) -> Result<Option<String>, OidcError> {
    if let Ok(token) = std::env::var(token_env)
        && let Some(token) = normalize_token(token)
    {
        return Ok(Some(token));
    }
    if let Ok(path) = std::env::var(file_env) {
        let token = std::fs::read_to_string(&path)
            .map_err(|error| OidcError::FetchFailed(format!("{file_error_prefix}: {error}")))?;
        if let Some(token) = normalize_token(token) {
            return Ok(Some(token));
        }
    }
    Ok(None)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
