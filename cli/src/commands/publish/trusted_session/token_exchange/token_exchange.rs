use super::super::{TrustedPublishSession, trusted_token_deadline};
use anyhow::Result;
use std::time::Duration;

const MAX_TRUSTED_EXCHANGE_RESPONSE_BYTES: usize = 4 * 1024;

pub(in crate::commands) async fn trusted_publish_session(
    registry: &str,
    package: &str,
    protocol: &str,
    explicit_audience: Option<&str>,
) -> Result<TrustedPublishSession> {
    let registry_url =
        url::Url::parse(registry).map_err(|_| crate::error::trusted_registry_url_invalid())?;
    if !registry_url.username().is_empty()
        || registry_url.password().is_some()
        || registry_url.fragment().is_some()
        || registry_url.query().is_some()
    {
        return Err(crate::error::trusted_registry_url_invalid());
    }
    let local_http = registry_url.scheme() == "http"
        && registry_url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    if registry_url.scheme() != "https" && !local_http {
        return Err(crate::error::trusted_registry_requires_https());
    }

    let audience = explicit_audience
        .map(str::to_owned)
        .or_else(|| std::env::var("MGC_OIDC_AUDIENCE").ok())
        .unwrap_or_else(|| registry_url.origin().ascii_serialization());
    if audience.trim().is_empty() {
        return Err(crate::error::trusted_audience_required());
    }
    let oidc_token = mgc_oidc::fetch::fetch_token(&audience).await?;
    let sigstore_oidc_token = mgc_oidc::fetch::fetch_sigstore_token().await?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    // Start conservatively before the request; server issuance and response transit consume TTL.
    // Bắt đầu tính trước khi gửi request; thời gian server cấp token và truyền response đều dùng TTL.
    let exchange_started = tokio::time::Instant::now();
    let response = client
        .post(format!(
            "{}/-/v1/trusted-publish/token",
            registry.trim_end_matches('/')
        ))
        .json(&serde_json::json!({
            "protocol": protocol,
            "package": package,
            "oidc_token": oidc_token,
            "sigstore_oidc_token": sigstore_oidc_token,
        }))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(crate::error::trusted_token_exchange_failed(
            response.status().as_u16(),
        ));
    }
    if response
        .content_length()
        .is_some_and(|length| length > MAX_TRUSTED_EXCHANGE_RESPONSE_BYTES as u64)
    {
        return Err(crate::error::trusted_token_response_invalid());
    }
    use futures_util::StreamExt as _;
    let mut stream = response.bytes_stream();
    let mut response_body = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| crate::error::trusted_token_response_invalid())?;
        if chunk.len() > MAX_TRUSTED_EXCHANGE_RESPONSE_BYTES.saturating_sub(response_body.len()) {
            return Err(crate::error::trusted_token_response_invalid());
        }
        response_body.extend_from_slice(&chunk);
    }
    let exchanged: serde_json::Value = serde_json::from_slice(&response_body)?;
    let token = exchanged
        .get("token")
        .and_then(serde_json::Value::as_str)
        .filter(|token| !token.is_empty() && !token.bytes().any(|byte| byte.is_ascii_control()))
        .ok_or_else(crate::error::trusted_token_response_invalid)?;
    let expires_in = exchanged
        .get("expires_in")
        .and_then(serde_json::Value::as_u64)
        .filter(|seconds| (1..=900).contains(seconds))
        .ok_or_else(crate::error::trusted_token_response_invalid)?;
    if exchanged.get("token_type").and_then(|value| value.as_str()) != Some("Bearer") {
        return Err(crate::error::trusted_token_response_invalid());
    }
    Ok(TrustedPublishSession {
        token: token.to_owned(),
        expires_at: trusted_token_deadline(exchange_started, Duration::from_secs(expires_in)),
        registry: registry.to_owned(),
        package: package.to_owned(),
        protocol: protocol.to_owned(),
        audience: explicit_audience.map(str::to_owned),
    })
}
