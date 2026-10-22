use super::error;
use crate::commands::template::{
    MAX_TEMPLATE_REDIRECTS, TEMPLATE_FETCH_TIMEOUT_SECS, template_auth_allowed,
    validate_template_download_url,
};
use anyhow::Result;

/// Re-evaluate credential scope on every hop; never let redirects forward auth implicitly.
/// Từng redirect phải xét lại origin/path; CDN ngoài registry không nhận credential.
pub(in crate::commands::template) async fn download_template_bytes(
    client: &reqwest::Client,
    mut target: url::Url,
    registry: &url::Url,
    authorization: Option<&str>,
    limit: usize,
) -> Result<Vec<u8>> {
    let started = std::time::Instant::now();
    let budget = std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS);
    for redirect in 0..=MAX_TEMPLATE_REDIRECTS {
        validate_template_download_url(&target, registry)?;
        let remaining = budget.saturating_sub(started.elapsed());
        if remaining.is_zero() {
            return Err(error::template_request_failed());
        }
        let mut request = client.get(target.clone()).timeout(remaining);
        if template_auth_allowed(registry, &target)
            && let Some(header) = authorization
        {
            request = request.header(reqwest::header::AUTHORIZATION, header);
        }
        let mut response = request.send().await.map_err(|error| {
            if error.is_connect() || error.is_timeout() {
                error::template_transport_unavailable()
            } else {
                error::template_request_failed()
            }
        })?;
        if response.status().is_redirection() {
            if redirect == MAX_TEMPLATE_REDIRECTS {
                return Err(error::template_redirect_invalid());
            }
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(error::template_redirect_invalid)?;
            target = target
                .join(location)
                .map_err(|_| error::template_redirect_invalid())?;
            continue;
        }
        if !response.status().is_success() {
            return Err(error::template_download_status(response.status().as_u16()));
        }
        if response
            .content_length()
            .is_some_and(|length| length > limit as u64)
        {
            return Err(error::template_download_too_large(limit));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| error::template_request_failed())?
        {
            if chunk.len() > limit.saturating_sub(bytes.len()) {
                return Err(error::template_download_too_large(limit));
            }
            bytes.extend_from_slice(&chunk);
        }
        return Ok(bytes);
    }
    Err(error::template_redirect_invalid())
}
