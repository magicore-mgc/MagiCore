//! Build the audience request and read a GitHub-issued JWT — tạo request audience và đọc JWT do GitHub cấp.
//!
use crate::error::OidcError;
use crate::fetch::endpoint::{fetch_json, secure_url};
use url::Url;

fn token_request_url(base: &str, audience: &str) -> Result<Url, OidcError> {
    let mut url = secure_url(base)?;
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| key != "audience")
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    url.query_pairs_mut()
        .clear()
        .extend_pairs(pairs)
        .append_pair("audience", audience);
    Ok(url)
}

/// GitHub Actions flow: `GET $ACTIONS_ID_TOKEN_REQUEST_URL&audience=…`
/// with `Authorization: bearer $ACTIONS_ID_TOKEN_REQUEST_TOKEN`.
/// Response shape: `{"value": "<jwt>"}`.
/// Gọi endpoint GitHub Actions bằng bearer token và đọc JWT từ trường `value`.
pub async fn fetch_github_token(audience: &str) -> Result<String, OidcError> {
    let base = std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").map_err(|_| OidcError::NoToken)?;
    let bearer = std::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN").map_err(|_| OidcError::NoToken)?;
    if base.trim().is_empty() || bearer.trim().is_empty() {
        return Err(OidcError::NoToken);
    }
    let url = token_request_url(&base, audience)?;
    let body = fetch_json(url, Some(bearer.trim())).await?;
    body.get("value")
        .and_then(|value| value.as_str())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| OidcError::FetchFailed("token response has no usable value".to_string()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
