//! Select generic or Sigstore token sources — chọn nguồn token chung hoặc Sigstore.
//!
use super::github_actions::fetch_github_token;
use super::token_source::{
    OIDC_TOKEN_ENV, OIDC_TOKEN_FILE_ENV, SIGSTORE_OIDC_TOKEN_ENV, SIGSTORE_OIDC_TOKEN_FILE_ENV,
    token_from_environment,
};
use crate::error::OidcError;

/// Fetch an OIDC JWT for `audience`: explicit env/file input wins (generic
/// CI), otherwise GitHub Actions is attempted. Never logs the token.
/// Lấy JWT cho audience: ưu tiên env/file, nếu không có thì thử GitHub Actions; không ghi token vào log.
pub async fn fetch_token(audience: &str) -> Result<String, OidcError> {
    if let Some(token) = token_from_environment(
        OIDC_TOKEN_ENV,
        OIDC_TOKEN_FILE_ENV,
        "cannot read OIDC token file",
    )? {
        return Ok(token);
    }
    fetch_github_token(audience).await
}

/// Fetch a separate token for Fulcio; a registry-audience token is not reusable.
/// Lấy token riêng cho Fulcio; token audience registry không thể dùng thay thế.
pub async fn fetch_sigstore_token() -> Result<String, OidcError> {
    if let Some(token) = token_from_environment(
        SIGSTORE_OIDC_TOKEN_ENV,
        SIGSTORE_OIDC_TOKEN_FILE_ENV,
        "cannot read Sigstore OIDC token file",
    )? {
        return Ok(token);
    }
    fetch_github_token("sigstore").await
}
